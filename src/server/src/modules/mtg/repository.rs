use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;
use tracing::error;
use turso::{Builder, Database};
use uuid::Uuid;

use deckmaster_domain::mtg::model::{Card, Deck};
use deckmaster_domain::mtg::service::MtgDataAccessLayer;
use deckmaster_domain::mtg::service::{FindCardsFilter, FindDecksFilter};

use crate::helpers::to_sql_query::ToSqlQuery;

const MTG_SQLITE_DB_PATH: &str = "data/mtg.sqlite";

impl ToSqlQuery for FindCardsFilter {
    fn to_sql_query(&self) -> String {
        let mut query = String::from(
            r#"
        SELECT
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
            deck_id
        FROM cards"#,
        );

        if self.deck_id.is_some() {
            query.push_str(" WHERE");
        }

        if let Some(deck_id) = &self.deck_id {
            // FIXME: This is vulnerable to SQL injection, but Turso doesn't support query builders
            // the values are parsed as UUIDs, so the risk is minimal
            query.push_str(&format!(" deck_id = '{deck_id}'"));
        }

        let limit = 20;
        let offset = (self.page.unwrap_or(1) - 1) * limit;
        query.push_str(&format!(" LIMIT {limit} OFFSET {offset}"));

        query
    }
}

impl ToSqlQuery for FindDecksFilter {
    fn to_sql_query(&self) -> String {
        let mut query = String::from(
            r#"
        SELECT
            id,
            name,
            code,
            release
        FROM decks"#,
        );

        if self.id.is_some() {
            query.push_str(" WHERE");
        }

        if let Some(id) = &self.id {
            // FIXME: This is vulnerable to SQL injection, but Turso doesn't support query builders
            // the values are parsed as UUIDs, so the risk is minimal
            query.push_str(&format!(" id = '{id}'"));
        }

        let limit = 20;
        let offset = (self.page.unwrap_or(1) - 1) * limit;
        query.push_str(&format!(" LIMIT {limit} OFFSET {offset}"));

        query
    }
}

#[derive(Clone)]
pub struct MtgRepository {
    db: Arc<Database>,
}

impl MtgRepository {
    pub async fn new() -> Result<Self> {
        let db = Builder::new_local(MTG_SQLITE_DB_PATH).build().await?;
        Ok(MtgRepository { db: Arc::new(db) })
    }
}

impl MtgDataAccessLayer for MtgRepository {
    async fn find_cards(&self, filter: FindCardsFilter) -> Result<Vec<Card>> {
        let conn = self.db.connect()?;
        let query = filter.to_sql_query();
        let mut rows = conn.query(&query, ()).await?;
        let mut cards = Vec::new();

        while let Some(row) = rows.next().await? {
            let Some(id) = row
                .get_value(0)?
                .as_text()
                .and_then(|s| Uuid::parse_str(s).ok())
            else {
                error!("Failed to parse UUID.");
                continue;
            };

            let Some(title) = row.get_value(1)?.as_text().map(|t| t.to_owned()) else {
                error!("Failed to get title.");
                continue;
            };

            let Some(number) = row.get_value(2)?.as_integer().map(|n| n.to_owned()) else {
                error!("Failed to get number.");
                continue;
            };

            let description = row.get_value(3)?.as_text().map(|d| d.to_string());

            let mana = row.get_value(4)?.as_text().map(|d| d.to_string());
            let mana = mana.map(|s| {
                s.split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            });

            let Some(kind) = row.get_value(5)?.as_text().map(|s| s.to_string()) else {
                error!("Failed to get kind.");
                continue;
            };

            let Some(rarity) = row.get_value(6)?.as_text().map(|s| s.to_string()) else {
                error!("Failed to get rarity.");
                continue;
            };

            let artist = row.get_value(7)?.as_text().map(|s| s.to_string());

            let power = row.get_value(8)?.as_text().map(|s| s.to_string());
            let toughness = row.get_value(9)?.as_text().map(|s| s.to_string());

            let Some(deck_id) = row
                .get_value(10)?
                .as_text()
                .and_then(|s| Uuid::parse_str(s).ok())
            else {
                error!("Failed to parse deck_id UUID.");
                continue;
            };

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

    async fn find_decks(&self, filter: FindDecksFilter) -> Result<Vec<Deck>> {
        let conn = self.db.connect()?;
        let query = filter.to_sql_query();
        let mut rows = conn.query(&query, ()).await?;
        let mut cards = Vec::new();

        while let Some(row) = rows.next().await? {
            let Some(id) = row
                .get_value(0)?
                .as_text()
                .and_then(|s| Uuid::parse_str(s).ok())
            else {
                error!("Failed to parse UUID.");
                continue;
            };

            let Some(name) = row.get_value(1)?.as_text().map(|t| t.to_owned()) else {
                error!("Failed to get name.");
                continue;
            };

            let Some(code) = row.get_value(2)?.as_text().map(|n| n.to_owned()) else {
                error!("Failed to get code.");
                continue;
            };

            // let Some(release) = row
            //     .get_value(3)?
            //     .as_text()
            //     .map(|s| DateTime::from_timestamp_nanos(&s.parse().unwrap_or_default()).ok())
            //     .flatten()
            //     .map(|dt| dt.with_timezone(&Utc))
            // else {
            //     error!("Failed to parse release date.");
            //     continue;
            // };

            cards.push(Deck {
                id,
                name,
                code,
                release: Utc::now(),
            });
        }

        Ok(cards)
    }
}
