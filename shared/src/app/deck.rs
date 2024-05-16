use std::collections::VecDeque;
use serde::{Deserialize, Serialize};
use crate::cards::Card;
use rand::{Rng};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Deck{
    //This may be better as a regular Vec
    pub deck: VecDeque<Card>
}

impl Deck {

    pub(crate) fn _add_card(&mut self, card: Card){
        self.deck.push_back(card);
    }

    pub(crate) fn shuffle(&mut self){
        let mut new_deck: VecDeque<Card> = VecDeque::with_capacity(self.deck.capacity());
        //May break on the last element?
        while !self.deck.is_empty() {
            //Surprisingly, since swap_remove and push_back are O(1), this should be O(N) (well, other than the random call)
            new_deck.push_back(self.deck.swap_remove_back(rand::thread_rng().gen_range(0..self.deck.len())).unwrap());
        }
        self.deck = new_deck;
    }
    
    pub(crate) fn top_card(&self) -> Option<&Card> {
        self.deck.back()
    }

    //This is way less efficient than keeping track of the current card and using get() on the top
    //card call; fix later.
    pub(crate) fn next_card(&mut self){
        if !self.deck.is_empty() {
            self.deck.rotate_left(1)
        }
    }

    //This is way less efficient than keeping track of the current card and using get() on the top
    //card call; fix later.
    pub(crate) fn previous_card(&mut self){
        if !self.deck.is_empty() {
            self.deck.rotate_right(1)
        }
    }

}