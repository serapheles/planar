pub mod cards;
//Pretty sure I need this, somewhere
// #[macro_use]
extern crate android_logger;
extern crate log;

use android_logger::{Config};
use crux_core::{render::Render, App};
use log::LevelFilter;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use crate::cards::{Card, CardType, get_card_vec};

fn native_activity_create() {
    // #[cfg(target_os = "android")]
    android_logger::init_once(Config::default().with_max_level(LevelFilter::Trace));
    log::trace!("Starting Crux logger");
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    Initialize,
    SetImage(String, u8),
    SetDatabase(String),
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Model {
    pub base_path: String,
    // pub active_deck: 
}

#[derive(Serialize, Deserialize)]
pub struct ViewModel {
    pub result: String,
}

#[cfg_attr(feature = "typegen", derive(crux_core::macros::Export))]
#[derive(crux_core::macros::Effect)]
#[effect(app = "Planar")]
pub struct Capabilities {
    render: Render<Event>,
}

#[derive(Default)]
pub struct Planar;

fn ensure_card_db(path: &str) -> Result<()>{
    let connection = Connection::open(path)?;
    let mut statement = connection.prepare(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='card_db'",
    )?;
    let mut res = statement.query([])?;
    
    if let Some(x) = res.next()? {
            log::info!("Row result: {:?}", x)
    } else {
        log::info!("Card_db not found.");
        //card_type should reference another table, but that's exhausting.
        //Could try serializing images to include as blobs, then passing those up?
        //Probably too much overhead to be worthwhile.
        connection.execute(
            "CREATE TABLE card_db (
                name    TEXT PRIMARY KEY,
                desc    TEXT NOT NULL,
                card_type   TEXT NOT NULL,
                image   INTEGER
            )",
            (),
        )?;
        
        let cards: Vec<Card> = get_card_vec();
        for card in cards {
            let card_type = match card.card_type {
                CardType::Plane => {"Plane"}
                CardType::Phenomenon => {"Phenomenon"}
            };
            connection.execute(
                "INSERT INTO card_db (name, desc, card_type) VALUES (?1, ?2, ?3)",
                               (&card.name, &card.desc, card_type),
            )?;
        }
    }
    
    Ok(())
}

fn set_image(path: &str, name: &str, id: u8) -> Result<()>{
    let connection = Connection::open(path)?;
    match connection.execute("UPDATE card_db SET image = ?1 WHERE name = ?2", (id, name)) {
        Ok(_) => {
            log::info!("Updated {} to have image id: {}.", name, id);
        }
        Err(_) => {
            log::warn!("Failed to update {} to image id {}", name, id);
        }
    }
    
    Ok(())
}

impl App for Planar {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Capabilities = Capabilities;

    fn update(&self, event: Self::Event, model: &mut Self::Model, caps: &Self::Capabilities) {
        log::trace!("Update: {event:?}. Model: {model:?}");

        match event {
            Event::Initialize => {
                native_activity_create();
            }
            Event::SetDatabase(val) => {
                log::info!("Database path: {}", val);
                model.base_path = val;
                let res = ensure_card_db(&model.base_path);
                log::warn!("Checking for card_db: {:?}", res.unwrap());
            }
            Event::SetImage(name, id) =>{
                let res = set_image(&model.base_path, name.as_str(), id);
                log::warn!("{:?}", res.unwrap());
            }
        };
        caps.render.render();
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            result: "To-do".to_string(),
        }
    }
}


