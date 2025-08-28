use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use sqlx::sqlite::SqliteRow;
use sqlx::{QueryBuilder, Row, SqlitePool};

use deckmaster_domain::mtg::model::{Card, Deck};
use deckmaster_domain::mtg::service::MtgDataAccessLayer;
use deckmaster_domain::mtg::service::{FindCardsFilter, FindDecksFilter};

#[derive(Clone)]
pub struct MtgRepository {
    db: Arc<SqlitePool>,
}

impl MtgRepository {
    pub async fn new(db: Arc<SqlitePool>) -> Result<Self> {
        Ok(MtgRepository { db })
    }
}

impl MtgDataAccessLayer for MtgRepository {
    async fn find_cards(&self, filter: FindCardsFilter) -> Result<Vec<Card>> {
        let mut conn = self.db.acquire().await?.detach();
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

        if let Some(deck_id) = filter.deck_id {
            query.push(" WHERE deck_id = ");
            query.push_bind(deck_id);
        }

        query.push(" FROM cards");
        let offset = (filter.page.unwrap_or(1).saturating_sub(1)) * 20;
        query.push(" LIMIT 20 OFFSET ");
        query.push_bind(offset as i64);

        let rows: Vec<SqliteRow> = query.build().fetch_all(&mut conn).await?;
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

        Ok(cards)
    }

    async fn find_decks(&self, _: FindDecksFilter) -> Result<Vec<Deck>> {
        let mut conn = self.db.acquire().await?.detach();
        let rows: Vec<SqliteRow> = sqlx::query(
            r#"SELECT
                id,
                name,
                code,
                release
            FROM decks"#,
        )
        .fetch_all(&mut conn)
        .await?;
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

        Ok(decks)
    }
}
