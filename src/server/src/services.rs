use std::sync::Arc;

use anyhow::Result;

use deckmaster_domain::mtg::{
    model::{Card, Deck},
    service::MtgService,
};

use crate::modules::mtg::repository::MtgRepository;

pub type SharedServices = Arc<Services>;

pub struct ServicesInit {
    pub cards: Vec<u8>,
    pub decks: Vec<u8>,
}

pub struct Services {
    pub mtg: MtgService<MtgRepository>,
}

impl Services {
    pub async fn new(init: ServicesInit) -> Result<Self> {
        let cards = csv::Reader::from_reader(init.cards.as_slice())
            .deserialize()
            .filter_map(|result| result.ok())
            .collect::<Vec<Card>>();
        let decks = csv::Reader::from_reader(init.decks.as_slice())
            .deserialize()
            .filter_map(|result| result.ok())
            .collect::<Vec<Deck>>();

        let mtg_repository = MtgRepository::new(cards, decks).await?;
        let mtg_repository = Arc::new(mtg_repository);
        let mtg = MtgService::new(mtg_repository);

        Ok(Self { mtg })
    }
}
