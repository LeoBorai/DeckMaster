use std::sync::Arc;

use anyhow::Result;
use bytes::Bytes;
use chrono::Utc;
use reqwest::{StatusCode, Url};
use sqlx::sqlite::SqliteRow;
use sqlx::{QueryBuilder, Row, SqlitePool};

use deckmaster_domain::common::query_set::{QuerySet, QuerySetMeta};
use deckmaster_domain::mtg::model::{Card, Deck};
use deckmaster_domain::mtg::service::{FindCardsFilter, FindDecksFilter};
use deckmaster_domain::mtg::service::{FindImageFilter, MtgDataAccessLayer};
use tracing::error;

#[derive(Clone)]
struct Counters {
    cards: u32,
    decks: u32,
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
        let _offset = page * 20;
        let mut query = QueryBuilder::new(
            r#"SELECT
                id,
                title,
                number,
                description,
                mana,
                kind,
                rarity,
                artist,
                power,
                toughness,
                deck_id"#,
        );

        query.push(" FROM cards");

        if let Some(id) = filter.id {
            query.push(format!(" WHERE  id = '{}'", id));
        }

        if let Some(title) = filter.title {
            let normalized = title.trim().to_ascii_lowercase();

            if filter.id.is_some() {
                query.push(format!(" AND  title LIKE '%{}%'", normalized));
            } else {
                query.push(format!(" WHERE  title LIKE '%{}%'", normalized));
            }
        }

        if let Some(unique) = filter.unique
            && unique
        {
            query.push(" GROUP BY deck_id");
        }

        query.push(" LIMIT 20");

        // FIXME: Needs review
        // if offset > 0 {
        //     query.push(" OFFSET ");
        //     query.push_bind(offset as i64);
        // }

        let rows: Vec<SqliteRow> = query.build().fetch_all(&mut conn).await.map_err(|err| {
            error!(?err, "An error ocurred fetching cards from database");
            err
        })?;
        let mut cards = Vec::new();

        for row in rows {
            let id = row.get::<String, _>(0).parse()?;
            let title = row.get::<String, _>(1);
            let number = row.get::<i64, _>(2);
            let description = row.get::<Option<String>, _>(3);
            let mana = row.get::<Option<String>, _>(4).map(|s| {
                s.split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            });
            let kind = row.get::<String, _>(5);
            let rarity = row.get::<String, _>(6);
            let artist = row.get::<Option<String>, _>(7);
            let power = row.get::<Option<String>, _>(8);
            let toughness = row.get::<Option<String>, _>(9);
            let deck_id = row.get::<String, _>(10).parse()?;

            cards.push(Card {
                id,
                title: title.to_owned(),
                number: number.to_owned(),
                description,
                mana,
                kind,
                rarity,
                artist,
                power,
                toughness,
                deck_id,
            });
        }

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
        let offset = page * 20;
        let mut query = QueryBuilder::new(
            r#"SELECT
                id,
                name,
                code,
                release
            FROM decks"#,
        );

        query.push(" LIMIT 20 OFFSET ");
        query.push_bind(offset as i64);

        let rows: Vec<SqliteRow> = query.build().fetch_all(&mut conn).await?;
        let mut decks = Vec::new();

        for row in rows {
            let id = row.get::<String, _>(0).parse()?;
            let name = row.get::<String, _>(1);
            let code = row.get::<String, _>(2);
            let release = row
                .get::<Option<chrono::DateTime<Utc>>, _>(3)
                .unwrap_or_default();

            decks.push(Deck {
                id,
                name,
                code,
                release,
            });
        }

        Ok(QuerySet::new(
            decks,
            QuerySetMeta {
                page,
                per_page: 20,
                total_pages: self.counters.decks.div_ceil(20),
            },
        ))
    }

    async fn find_image(&self, filter: FindImageFilter) -> Result<Bytes> {
        let image_url = self.storage_url.join(&format!(
            "magic-the-gathering/images/cards/{}/{}.jpg",
            filter.deck_id, filter.card_id
        ))?;
        let response = reqwest::get(image_url).await?;

        if response.status() == StatusCode::OK {
            let bytes = response.bytes().await?;
            return Ok(bytes);
        }

        Err(anyhow::anyhow!(
            "Image not found. Status: {}",
            response.status()
        ))
    }
}
