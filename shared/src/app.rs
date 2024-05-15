pub mod cards;
mod deck;

extern crate android_logger;
extern crate log;

use std::collections::VecDeque;
use std::error::Error;
use android_logger::{Config};
use crux_core::{render::Render, App};
use log::LevelFilter;
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use crate::app::deck::Deck;
use crate::cards::{Card, CardType, get_card_type, get_card_vec};

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
    AddDeckToDB,
    GetDeckFromDB,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Model {
    pub base_path: String,
    pub active_deck: Deck,
}

#[derive(Serialize, Deserialize)]
pub struct ViewModel {
    pub active_card: String,
}

#[cfg_attr(feature = "typegen", derive(crux_core::macros::Export))]
#[derive(crux_core::macros::Effect)]
#[effect(app = "Planar")]
pub struct Capabilities {
    render: Render<Event>,
}

#[derive(Default)]
pub struct Planar;

fn add_deck_to_db(path: &str, deck_name: &str, deck: Deck) -> Result<()>{
    let deck_json = serde_json::to_string(&deck).unwrap();
    let connection = Connection::open(path)?;
    match connection.execute("INSERT INTO deck_db (name, deck) VALUES (?1, ?2)
    ON CONFLICT(name) DO UPDATE SET deck = ?2", (deck_name, deck_json)) {
        Ok(_) => {
            log::info!("Added {} to database", deck_name, );
        }
        Err(_) => {
            log::warn!("Failed to add {} to database", deck_name);
        }
    }
    Ok(())
}

fn get_card_from_db(path: &str, card_name: &str) -> Result<Card,  Box<dyn Error>>{
    let connection = Connection::open(path)?;
    let mut statement = connection.prepare("SELECT * FROM card_db WHERE name=?1")?;

    let mut res = statement.query([card_name])?;

    if let Some(row) = res.next()? {
        let card = Card {
            card_type: get_card_type(row.get(1)?),
            desc: row.get(2)?,
            image: String::from(""),
            name: row.get(0)?,
        };
        Ok(card)
    } else {
        Err(Box::from("Fail"))
    }
}

fn get_deck_from_db(path: &str, deck_name: &str) -> Result<Deck>{
    let connection = Connection::open(path)?;
    let mut statement = connection.prepare("SELECT deck FROM deck_db WHERE name=?1")?;
    let mut rows = statement.query([deck_name])?;

    //This is ugly, do better
    if let Some(row) = rows.next()? {
        let res: String = row.get(0)?;
        let deck: Deck = serde_json::from_str(res.as_str()).unwrap();
        Ok(deck)
    } else {
        log::error!("Failed to get {} from db!", deck_name);
        Ok(Deck {
            //Should probably warn instead, especially before things are tested.
            deck: Default::default()
        })
    }
}

fn ensure_card_db(path: &str) -> Result<()>{
    let connection = Connection::open(path)?;
    let mut statement = connection.prepare(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='card_db'",
    )?;
    let mut res = statement.query([])?;
    
    if let Some(row) = res.next()? {
            log::info!("Row result: {:?}", row)
    } else {
        log::info!("card_db not found.");
        //card_type should reference another table, but that's exhausting.
        //Could try serializing images to include as blobs, then passing those up?
        //Probably too much overhead to be worthwhile.
        connection.execute(
            "CREATE TABLE card_db (
                name    TEXT PRIMARY KEY,
                card_type   TEXT NOT NULL,
                desc    TEXT NOT NULL,
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

fn ensure_deck_db(path: &str) -> Result<()> {
    let connection = Connection::open(path)?;
    let mut statement = connection.prepare(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='deck_db'",
    )?;
    let mut res = statement.query([])?;

    if let Some(x) = res.next()? {
        log::info!("Row result: {:?}", x)
    } else {
        log::info!("deck_db not found.");

        //This shouldn't use JSON(TEXT), it should probably use a separate table, but relational databases
        //are dumb, and I don't want a separate table for each deck.
        connection.execute(
            "CREATE TABLE deck_db (
                name    TEXT PRIMARY KEY,
                deck    TEXT
            )",
            (),
        )?;
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

fn set_default_deck() -> Deck{
    let pile = get_card_vec();
    let mut deck = Deck { deck: VecDeque::from(pile) };
    deck.shuffle();
    deck
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
                model.active_deck = set_default_deck();
            }
            Event::SetDatabase(val) => {
                log::info!("Database path: {}", val);
                model.base_path = val;
                let res = ensure_card_db(&model.base_path);
                log::warn!("Checking for card_db: {:?}", res.unwrap());
                let res2 = ensure_deck_db(&model.base_path);
                log::warn!("Checking for deck_db: {:?}", res2.unwrap());

            }
            Event::SetImage(name, id) =>{
                let res = set_image(&model.base_path, name.as_str(), id);
                log::warn!("{:?}", res.unwrap());
            }
            Event::AddDeckToDB => {
                let temp_deck = Deck{
                    deck: Default::default(),
                };
                add_deck_to_db(&model.base_path, "temp", temp_deck).expect("TODO: panic message");
            }
            Event::GetDeckFromDB => {
                let _temp_deck = get_deck_from_db(&model.base_path, "temp_deck");
            }
        };
        caps.render.render();
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        let active_card = match model.active_deck.top_card() {
            Some(card) => {card.name.clone()}
            None => { "cat" }.parse().unwrap()
        };
        ViewModel {
            active_card
        }
    }
}


#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use super::{add_deck_to_db, ensure_card_db, ensure_deck_db, get_card_from_db};
    use crate::app::deck::Deck;
    use crate::cards::get_card_vec;

    #[test]
    fn test_db_setup(){
        let path = "./test_db.sqlite";
        match ensure_card_db(path) {
            Ok(_) => {println!("card_db setup called without errors")}
            Err(err) => {println!("Error when calling card_db setup: {}", err)}
        }

        match get_card_from_db(path, "Academy at Tolaria West"){
            Ok(x) => {println!("Card found: {}", x.name)}
            Err(err) => {println!("Error getting card: {}", err)}
        }

        match ensure_deck_db(path) {
            Ok(_) => {println!("deck_db setup called without errors")}
            Err(err) => {println!("Error when calling deck_db setup: {}", err)}
        }
    }

    #[test]
    fn test_create_deck(){
        let pile = get_card_vec();
        let mut deck = Deck { deck: VecDeque::new() };
        for card in pile {
            deck.add_card(card);
            match deck.top_card() {
                Some(x) => println!("Current Top Card: {}", x.name),
                None => println!("No top card found!")
            }
            //assert top = card?
        }
        deck.shuffle();
        for (i, card) in deck.deck.iter().clone().enumerate(){
            println!("Card {}: {}", i, card.name);
        }

        let path = "./test_db.sqlite";
        match ensure_deck_db(path) {
            Ok(_) => {println!("DB setup called without errors")}
            Err(err) => {println!("Error when calling DB setup: {}", err)}
        }
        add_deck_to_db(path, "test_deck", deck).expect("Something broke");
    }
}