use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use serde::{Deserialize, Serialize};

const _PLANE_FILE: &[u8] = include_bytes!("../../../resources/planes.json");
const _PHENOMENON_FILE: &[u8] = include_bytes!("../../../resources/phenomenon.json");

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Card {
    pub card_type: CardType,
    pub desc: String,
    pub image: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default)]
pub enum CardType {
    #[default]
    Plane,
    Phenomenon,
}

pub fn get_card_type(card_type: String) -> CardType {
    if card_type == "Plane" {
        CardType::Plane
    } else {
        CardType::Phenomenon
    }
}

//Should return a result that is checked
pub fn get_card_vec() -> Vec<Card>{
    let mut cards_planes: Vec<Card> = serde_json::from_slice(_PLANE_FILE).unwrap();
    let mut cards_phenomenon: Vec<Card> = serde_json::from_slice(_PHENOMENON_FILE).unwrap();
    cards_planes.append(&mut cards_phenomenon);
    cards_planes
}

pub fn _read_from_file(card_type: CardType) -> Result<(), Box<dyn Error>>{
    let card_file = match card_type {
        CardType::Plane => {"resources/planes.json"}
        CardType::Phenomenon => {"resources/phenomenon.json"}
    };

    let raw = fs::read_to_string(card_file).unwrap();
    let json: Vec<Card> = serde_json::from_str(&raw).unwrap();
    for val in json{
        //This is absurdly inefficient
        println!("resourceMap[\"{}\"] = R.drawable.{}", val.name, val.name.replace(' ', "_").replace('\'', "").replace('-', "_").to_ascii_lowercase());
    }
    Ok(())
}

pub fn _initialize_cards(card_type: CardType){
    let card_file = match card_type {
        CardType::Plane => {"resources/Planes.txt"}
        CardType::Phenomenon => {"resources/Phenomenon.txt"}
    };
    let card_lines = BufReader::new(fs::File::open(card_file).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();
    let mut cards = Vec::new();
    {
        let mut iter = IntoIterator::into_iter(card_lines);
        //Untested since changes, may not end (or work) properly.
        #[allow(clippy::while_let_loop)] loop {
            let name = match iter.next() {
                Some(val) => val,
                None => break,
            };
            let desc = match iter.next() {
                Some(val) => val,
                None => break,
            };
            match card_type {
                CardType::Plane => {
                    let image = format!("resources/planes/{}.png", name.replace(' ', "_"));
                    cards.push(Card {
                        card_type,
                        desc,
                        image,
                        name,
                    })
                }
                CardType::Phenomenon => {
                    let image = format!("resources/phenomenon/{}.png", name.replace(' ', "_"));
                    cards.push(Card {
                        card_type,
                        desc,
                        image,
                        name,
                    })
                }
            }
        };
    }
    
    let file = match card_type {
        CardType::Plane => {File::create("resources/planes.json")}
        CardType::Phenomenon => {File::create("resources/phenomenon.json")}
    };

    let mut writer = BufWriter::new(file.unwrap());
    serde_json::to_writer(&mut writer, &cards).unwrap();
    writer.flush().unwrap();
}