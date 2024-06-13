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
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    Initialize(String),
    AddDeckToDB,
    GetDeckFromDB(String),
    NextCard,
    PreviousCard,
    SetDatabase,
    SetImage(String, u8),
    ShuffleActive,
    None,
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
pub struct Capabilities {
    render: Render<Event>,
    //Pretty sure I should make the db stuff a capability
}

#[derive(Default)]
pub struct Planar;

//Adds a deck to the database. Converts the deck to a json string, then adds as text.
fn add_deck_to_db(path: &str, deck_name: &str, deck: Deck) -> Result<(), Box<dyn Error>>{
    let deck_json = serde_json::to_string(&deck)?;
    let connection = Connection::open(path)?;
    match connection.execute("INSERT INTO deck_db (name, deck) VALUES (?1, ?2)
    ON CONFLICT(name) DO UPDATE SET deck = ?2", (deck_name, deck_json)) {
        Ok(_) => {
            Ok(())
        }
        Err(err) => {
            Err(err.into())
        }
    }
}

//Gets the information for a card from the database and creates an object for it.
//Honestly may be better to load all the cards in a datastructure at load and pull from there
//Currently sets all images to empty strings, because I'm not sure how I want to handle that field
fn _get_card_from_db(path: &str, card_name: &str) -> Result<Card,  Box<dyn Error>>{
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
        Err(Box::from("Failed to retrieve card from database."))
    }
}

//Gets the information for a deck, stored as a JSON, from the database and returns it as a deck object
fn get_deck_from_db(path: &str, deck_name: &str) -> Result<Deck, Box<dyn Error>>{
    let connection = Connection::open(path)?;
    let mut statement = connection.prepare("SELECT deck FROM deck_db WHERE name=?1")?;
    let mut rows = statement.query([deck_name])?;

    if let Some(row) = rows.next()? {
        let res: String = row.get(0)?;
        let deck: Deck = serde_json::from_str(res.as_str())?;
        Ok(deck)
    } else {
        Err(Box::from("Failed to retrieve deck from database."))
    }
}

//Ensures the database of cards exists. In most cases, this probably only needs to run once ever.
fn ensure_card_db(path: &str) -> Result<(), Box<dyn Error>>{
    let connection = Connection::open(path)?;
    let mut statement = connection.prepare(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='card_db'",
    )?;
    let mut res = statement.query([])?;
    
    if let Some(row) = res.next()? {
        //Do I want/need this?
        log::info!("Row result: {:?}", row)
    } else {
        log::warn!("card_db not found.");
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
        log::warn!("card_db created.");
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

//Ensures the database of decks exists. In most cases, this probably only needs to run once ever.
fn ensure_deck_db(path: &str) -> Result<(), Box<dyn Error>> {
    let connection = Connection::open(path)?;
    let mut statement = connection.prepare(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='deck_db'",
    )?;
    let mut res = statement.query([])?;

    if let Some(x) = res.next()? {
        log::info!("Row result: {:?}", x)
    } else {
        log::warn!("deck_db not found.");
        
        //This should probably use a reference to a separate table, not a JSON (text), but 
        //relational databases are dumb, and I don't want a separate table for each deck.
        //THAT SAID, this ends up being a huge waste of space as more decks are added. Unfortunately,
        //referencing another table is probably the right thing to do (perhaps via a unique card id)
        //((Could be set/number, e.g. "OHOP001 : Academy at Tolaria West"))
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

//Sets the image field for a card to the passed value.
//Not entirely sure if I want to be doing this, but it may be useful later.
fn set_image(path: &str, name: &str, id: u8) -> Result<(), Box<dyn Error>>{
    let connection = Connection::open(path)?;
    match connection.execute("UPDATE card_db SET image = ?1 WHERE name = ?2", (id, name)) {
        Ok(_) => {
            Ok(())
        }
        Err(err) => {
            Err(err.into())
        }
    }
}

//Sets a default deck of all the cards.
//Useful until deck building is properly set up and working.
fn set_default_deck() -> Deck{
    log::debug!("Setting default deck.");
    let mut deck = Deck { deck: VecDeque::from(get_card_vec()) };
    deck.shuffle();
    deck
}

impl App for Planar {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Capabilities = Capabilities;

    fn update(&self, event: Self::Event, model: &mut Self::Model, caps: &Self::Capabilities) {
        // log::debug!("Update: {event:?}. Model: {model:?}");

        match event {
            Event::Initialize(path) => {
                //Should probably set the database path here as well.
                log::info!("Starting Crux logger");
                native_activity_create();
                model.base_path = path;
                Self.update(Event::SetDatabase, model, caps);
                //May want to ensure there isn't an active deck first.
                model.active_deck = set_default_deck();
                log::info!("Set default deck.");
                caps.render.render();
            }
            //TODO: have this take an actual deck, probably as a vec<String>
            Event::AddDeckToDB => {
                let temp_deck = Deck{
                    deck: Default::default(),
                };
                match add_deck_to_db(&model.base_path, "temp", temp_deck) {
                    Ok(_) => {
                        log::info!("Added deck to database.");
                    }
                    Err(err) => {
                        log::error!("Failed to add deck to database: {}", err);
                    }
                }
            }
            Event::GetDeckFromDB(name) => {
                match get_deck_from_db(&model.base_path, name.as_str()){
                    Ok(_) => {
                        //TODO: something here.
                    }
                    Err(err) => {
                        log::error!("Failed to retrieve deck from database: {}", err);
                    }
                }
            }
            Event::NextCard => {
                model.active_deck.next_card();
                caps.render.render();
            }
            Event::PreviousCard => {
                model.active_deck.previous_card();
                caps.render.render();
            }
            //This should probably be ensuring the card db as well
            Event::SetDatabase => {
                match ensure_card_db(&model.base_path) {
                    Ok(_) => {
                        log::info!("Deck database assured.")
                    }
                    Err(err) => {
                        log::error!("Failed to ensure deck database: {}", err);
                    }
                }
                match ensure_deck_db(&model.base_path) {
                    Ok(_) => {
                        log::info!("Deck database assured.")
                    }
                    Err(err) => {
                        log::error!("Failed to ensure deck database: {}", err);
                    }
                }
            }
            Event::SetImage(name, id) =>{
                match set_image(&model.base_path, name.as_str(), id){
                    Ok(_) => {
                        log::info!("Updated image field for {}", name);
                    }
                    Err(err) => {
                        log::error!("Failed to set card image field: {}", err);
                    }
                }
            }
            Event::ShuffleActive => {
                model.active_deck.shuffle();
                //Not entirely sure if this needs to re-render.
                caps.render.render();
            }
            Event::None => {caps.render.render();}
        };
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

//These all passed last time I ran them, but I've made changes since.
#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use super::{add_deck_to_db, ensure_card_db, ensure_deck_db, _get_card_from_db};
    use crate::app::deck::Deck;
    use crate::cards::get_card_vec;

    #[test]
    fn test_db_setup(){
        let path = "./test_db.sqlite";
        match ensure_card_db(path) {
            Ok(_) => {println!("card_db setup called without errors")}
            Err(err) => {println!("Error when calling card_db setup: {}", err)}
        }

        match _get_card_from_db(path, "Academy at Tolaria West"){
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
            deck._add_card(card);
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
