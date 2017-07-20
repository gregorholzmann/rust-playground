extern crate rand;

use super::Card;
use rand::Rng;

pub fn generate_deck() -> Vec<Card> {
	
	let suits = ["♠", "♣", "♥", "♦"];
	let card_number = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"];
	let mut deck: Vec<Card> = Vec::new();

	for suit in &suits {
		for number in &card_number {
			let card = Card {
				suit: suit,
				value: number,
			};
			deck.push(card);
		}
	}
	rand::thread_rng().shuffle(&mut deck);
	deck
}