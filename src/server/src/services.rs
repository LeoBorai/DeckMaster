use std::sync::Arc;

use anyhow::{Result, anyhow};

use deckmaster_domain::mtg::{
    model::{Card, Deck},
    service::MtgService,
};
use reqwest::get;
use worker::console_log;
use worker::kv::KvStore;

use crate::modules::mtg::repository::MtgRepository;

pub type SharedServices = Arc<Services>;

const CARDS_CACHE_CHUNKS: usize = 95;

enum Cache {
    Hit { cards: Vec<Card>, decks: Vec<Deck> },
    Miss,
}

struct Datasets {
    cards: Vec<Card>,
    decks: Vec<Deck>,
}

pub struct Resources {
    pub cards: Vec<Card>,
    pub decks: Vec<Deck>,
}

pub struct Services {
    pub mtg: MtgService<MtgRepository>,
}

impl Services {
    pub async fn new(kv: &KvStore) -> Result<Self> {
        let Resources { cards, decks } = Self::fetch_resources(kv).await?;
        let mtg_repository = MtgRepository::new(cards, decks).await?;
        let mtg_repository = Arc::new(mtg_repository);
        let mtg = MtgService::new(mtg_repository);

        Ok(Self { mtg })
    }

    async fn fetch_resources(kv: &KvStore) -> Result<Resources> {
        let datasets = Self::fetch_datasets().await?;
        Self::store_cache(kv, &datasets).await?;
        let (cards, decks) = {
            match Self::fetch_cache(kv).await? {
                Cache::Hit { cards, decks } => {
                    console_log!("Caché HIT.");
                    (cards, decks)
                }
                Cache::Miss => {
                    console_log!("Caché MISS.");
                    let datasets = Self::fetch_datasets().await?;
                    Self::store_cache(kv, &datasets).await?;
                    (datasets.cards, datasets.decks)
                }
            }
        };

        Ok(Resources { cards, decks })
    }

    async fn fetch_datasets() -> Result<Datasets> {
        let cards = get(env!("CARDS_DATASET_URL")).await?.text().await?;
        let decks = get(env!("DECKS_DATASET_URL")).await?.text().await?;

        let cards = csv::Reader::from_reader(cards.as_bytes())
            .deserialize()
            .filter_map(|result| result.ok())
            .collect::<Vec<Card>>();
        let decks = csv::Reader::from_reader(decks.as_bytes())
            .deserialize()
            .filter_map(|result| result.ok())
            .collect::<Vec<Deck>>();

        console_log!("Got {} Cards", cards.len());
        console_log!("Got {} Decks", decks.len());

        Ok(Datasets { cards, decks })
    }

    async fn fetch_cache(kv: &KvStore) -> Result<Cache> {
        let Ok(Some(cards_cache)) = Self::fetch_cards_cache(kv).await else {
            return Ok(Cache::Miss);
        };

        let Ok(Some(decks_cache)) = kv.get("decks").bytes().await else {
            return Ok(Cache::Miss);
        };

        if cards_cache.is_empty() || decks_cache.is_empty() {
            return Ok(Cache::Miss);
        }

        let cards = csv::Reader::from_reader(cards_cache.as_slice())
            .deserialize()
            .filter_map(|result| result.ok())
            .collect::<Vec<Card>>();
        let decks = csv::Reader::from_reader(decks_cache.as_slice())
            .deserialize()
            .filter_map(|result| result.ok())
            .collect::<Vec<Deck>>();

        Ok(Cache::Hit { cards, decks })
    }

    async fn fetch_cards_cache(kv: &KvStore) -> Result<Option<Vec<u8>>> {
        let mut cards = Vec::new();

        for idx in 0..CARDS_CACHE_CHUNKS {
            let key = format!("cards_{idx}");
            let chunk = match kv.get(&key).bytes().await {
                Ok(Some(data)) => data,
                Ok(None) => break,
                Err(err) => {
                    return Err(anyhow!("Failed to retrieve cards chunk from caché. {err}"));
                }
            };
            cards.extend_from_slice(&chunk);
        }

        if cards.is_empty() {
            Ok(None)
        } else {
            Ok(Some(cards))
        }
    }

    async fn store_cache(kv: &KvStore, datasets: &Datasets) -> Result<()> {
        let mut csv_writer = csv::Writer::from_writer(vec![]);
        console_log!("Storing cards: {}", datasets.cards.len());
        for card in &datasets.cards {
            csv_writer
                .serialize(card)
                .map_err(|err| anyhow!("Failed to serialize card for caché. {err}"))?;
        }

        let csv = String::from_utf8(csv_writer.into_inner()?)?;
        let chunks = csv.lines().collect::<Vec<&str>>();
        console_log!("Lines: {}", chunks.len());
        let chunks = chunks.chunks(1000);

        console_log!("Total Chunks {}", chunks.len());
        for (idx, chunk) in chunks.enumerate() {
            kv.put(&format!("cards_{idx}"), chunk.join("\n"))
                .map_err(|err| anyhow!("Failed to store cards chunk in caché. {err}"))?
                .execute()
                .await
                .map_err(|err| anyhow!("Failed to execute cards chunk caché store. {err}"))?;
        }

        let mut csv_writer = csv::Writer::from_writer(vec![]);

        for deck in &datasets.decks {
            csv_writer
                .serialize(deck)
                .map_err(|err| anyhow!("Failed to serialize deck for caché. {err}"))?;
        }

        let csv = String::from_utf8(csv_writer.into_inner()?)?;

        kv.put("decks", csv.as_str())
            .map_err(|err| anyhow!("Failed to store cards in caché. {err}"))?
            .execute()
            .await
            .map_err(|err| anyhow!("Failed to execute decks caché store. {err}"))?;

        Ok(())
    }
}
