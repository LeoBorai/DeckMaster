use std::pin::Pin;
use std::sync::Arc;

use anyhow::{Context, Result};
use bytes::Bytes;
use chrono::{DateTime, Utc};
use futures_util::Stream;
use reqwest::{StatusCode, Url};
use sea_query::{Expr, ExprTrait, Iden, Query, SqliteQueryBuilder};
use sea_query_sqlx::SqlxBinder;
use sqlx::{FromRow, SqlitePool};
use tracing::error;
use uuid::Uuid;

use deckmaster_domain::common::query_set::{QuerySet, QuerySetMeta};
use deckmaster_domain::mtg::model::{Card, Deck};
use deckmaster_domain::mtg::service::{FindCardsFilter, FindDecksFilter};
use deckmaster_domain::mtg::service::{FindImageFilter, MtgDataAccessLayer};

const PAGE_SIZE: u64 = 20;

#[derive(Clone)]
struct Counters {
    cards: u32,
    decks: u32,
}

#[derive(Iden)]
enum Cards {
    Table,
    Id,
    Title,
    Number,
    Description,
    Mana,
    Kind,
    Rarity,
    Artist,
    Power,
    Toughness,
    DeckId,
}

#[derive(Debug, FromRow)]
struct CardRow {
    pub id: String,
    pub title: String,
    pub number: i64,
    pub description: Option<String>,
    pub mana: Option<String>,
    pub kind: String,
    pub rarity: String,
    pub artist: Option<String>,
    pub power: Option<String>,
    pub toughness: Option<String>,
    pub deck_id: String,
}

impl TryFrom<CardRow> for Card {
    type Error = anyhow::Error;

    fn try_from(row: CardRow) -> Result<Self> {
        let id = Uuid::parse_str(&row.id)?;
        let deck_id = Uuid::parse_str(&row.deck_id)?;

        Ok(Card {
            id,
            title: row.title,
            number: row.number,
            description: row.description,
            mana: row.mana.map(|s| {
                s.split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            }),
            kind: row.kind,
            rarity: row.rarity,
            artist: row.artist,
            power: row.power,
            toughness: row.toughness,
            deck_id,
        })
    }
}

#[derive(Iden)]
enum Decks {
    Table,
    Id,
    Name,
    Code,
    Release,
}

#[derive(Debug, FromRow)]
struct DeckRow {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub release: DateTime<Utc>,
}

impl TryFrom<DeckRow> for Deck {
    type Error = anyhow::Error;

    fn try_from(row: DeckRow) -> Result<Self> {
        Ok(Deck {
            id: row.id,
            name: row.name,
            code: row.code,
            release: row.release,
        })
    }
}

#[derive(Clone)]
pub struct MtgRepository {
    db: Arc<SqlitePool>,
    storage_url: Url,
    counters: Counters,
}

impl MtgRepository {
    pub async fn new(db: Arc<SqlitePool>, storage_url: Url) -> Result<Self> {
        let mut repo = MtgRepository {
            db,
            storage_url,
            counters: Counters { cards: 0, decks: 0 },
        };

        repo.count_tables().await?;

        Ok(repo)
    }

    // The `mtg` table is immutable, so we can cache the count of rows
    // to avoid running a COUNT(*) query every time we need it.
    async fn count_tables(&mut self) -> Result<()> {
        let mut conn = self.db.acquire().await?.detach();
        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM cards")
            .fetch_one(&mut conn)
            .await?;
        self.counters.cards = row.0 as u32;

        let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM decks")
            .fetch_one(&mut conn)
            .await?;
        self.counters.decks = row.0 as u32;

        Ok(())
    }
}

impl MtgDataAccessLayer for MtgRepository {
    async fn find_cards(&self, filter: FindCardsFilter) -> Result<QuerySet<Card>> {
        let mut conn = self.db.acquire().await?.detach();
        let page = filter.page.unwrap_or(0);
        let cols = if filter.unique.unwrap_or(false) {
            [Cards::DeckId]
        } else {
            [Cards::Id]
        };
        let (sql, values) = Query::select()
            .columns([
                Cards::Id,
                Cards::Title,
                Cards::Number,
                Cards::Description,
                Cards::Mana,
                Cards::Kind,
                Cards::Rarity,
                Cards::Artist,
                Cards::Power,
                Cards::Toughness,
                Cards::DeckId,
            ])
            .from(Cards::Table)
            .and_where_option(filter.id.map(|id| Expr::col(Cards::Id).eq(id.to_string())))
            .and_where_option(
                filter
                    .deck_id
                    .map(|deck_id| Expr::col(Cards::DeckId).eq(deck_id.to_string())),
            )
            .and_where_option(filter.skip.map(|skip| Expr::col(Cards::Id).is_not(skip)))
            .and_where_option(filter.title.map(|title| {
                Expr::col(Cards::Title).like(format!("%{}%", title.replace('%', "\\%")))
            }))
            .group_by_columns(cols)
            .limit(PAGE_SIZE)
            .build_sqlx(SqliteQueryBuilder);

        let cards = sqlx::query_as_with::<_, CardRow, _>(&sql, values.clone())
            .fetch_all(&mut conn)
            .await
            .context("Failed to fetch cards")?;

        let cards: Vec<Card> = cards
            .into_iter()
            .filter_map(|r| {
                Card::try_from(r)
                    .map_err(|err| error!(?err, "Failed to convert row into entity for card."))
                    .ok()
            })
            .collect();

        Ok(QuerySet::new(
            cards,
            QuerySetMeta {
                page,
                per_page: 20,
                total_pages: self.counters.cards.div_ceil(20),
            },
        ))
    }

    async fn find_decks(&self, filter: FindDecksFilter) -> Result<QuerySet<Deck>> {
        let mut conn = self.db.acquire().await?.detach();
        let page = filter.page.unwrap_or(1);
        let (sql, values) = Query::select()
            .columns([Decks::Id, Decks::Name, Decks::Code, Decks::Release])
            .from(Decks::Table)
            .group_by_col(Decks::Table)
            .limit(PAGE_SIZE)
            .build_sqlx(SqliteQueryBuilder);

        let decks = sqlx::query_as_with::<_, DeckRow, _>(&sql, values.clone())
            .fetch_all(&mut conn)
            .await
            .context("Failed to fetch decks")?;

        let decks: Vec<Deck> = decks
            .into_iter()
            .filter_map(|r| {
                Deck::try_from(r)
                    .map_err(|err| error!(?err, "Failed to convert row into entity for card."))
                    .ok()
            })
            .collect();

        Ok(QuerySet::new(
            decks,
            QuerySetMeta {
                page,
                per_page: 20,
                total_pages: self.counters.decks.div_ceil(20),
            },
        ))
    }

    async fn find_image(
        &self,
        filter: FindImageFilter,
    ) -> Result<Option<Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send + Sync>>>>
    {
        let image_url = self.storage_url.join(&format!(
            "magic-the-gathering/images/cards/{}/{}.jpg",
            filter.deck_id, filter.card_id
        ))?;
        let response = reqwest::get(image_url).await?;

        if response.status() == StatusCode::OK {
            return Ok(Some(Box::pin(response.bytes_stream())));
        }

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        Err(anyhow::anyhow!(
            "Failed to fetch image from storage. Status: {}",
            response.status()
        ))
    }
}
