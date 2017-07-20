extern crate rand;

pub mod deck_generator;

use std::io;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Card {
    suit: &'static str,
    value: &'static str,
}

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
    total_value: u8,
}

impl Hand {
    fn calc_total(&mut self) {
        let face_cards = ["A", "J", "Q", "K"];
        self.total_value = 0;
        for card in &self.cards {
            let mut num_value: u8 = 0;
            if face_cards.contains(&card.value) {
                num_value = 10;
            } else {
                num_value = card.value.parse::<u8>().unwrap();
            }
            self.total_value = self.total_value + num_value;
        };
    }
}

static BUST_THRESHOLD: u8 = 21; 

fn main() {
    println!("Welcome to Blackjack!");

    let mut dealer_hand = Hand {
        cards: Vec::<Card>::new(),
        total_value: 0,
    };
    let mut player_hand = Hand {
        cards: Vec::<Card>::new(),
        total_value: 0,
    };

    let mut deck = deck_generator::generate_deck();
    player_hand = draw_card(player_hand, &mut deck, 2);
    dealer_hand = draw_card(dealer_hand, &mut deck, 2);

    loop {
        println!("Your hand is: {}", readable_cards(&player_hand));
        println!("Dealer card showing is: {}{}", dealer_hand.cards[0].value, dealer_hand.cards[0].suit);

        if player_hand.total_value > BUST_THRESHOLD || player_hand.total_value == BUST_THRESHOLD {
            end_game(player_hand, dealer_hand);
            break;
        };

        let mut user_input = String::new();    

        println!("Would you like to hit? Enter [y] or [n].");

        io::stdin().read_line(&mut user_input)
            .expect("Please enter [y] or [n].");
        
        let will_hit: bool = match user_input.trim() {
            "y" => true,
            "n" => false,
            _ => continue,
        };
        
        if will_hit {
            player_hand = draw_card(player_hand, &mut deck, 1);
        } else {
            calc_dealer_hand(player_hand, dealer_hand, &mut deck);
            break;
        }
    }
}

fn draw_card(mut hand: Hand, deck: &mut Vec<Card>, number_of_cards: u8) -> Hand {
    for x in 0..number_of_cards {
        let drawn_card = deck.pop();
        match drawn_card {
            Some(x) => {
                hand.cards.push(x);
                break;
            },
            None    => break
        }
    }
    hand.calc_total();
    println!("{:?}", hand);
    hand
}

fn end_game(player_hand: Hand, dealer_hand: Hand) {
    println!("Final Player Hand: {:?}", player_hand.cards);
    println!("Final Dealer Hand: {:?}", dealer_hand.cards);
    if player_hand.total_value > BUST_THRESHOLD {
        println!("You bust! Dealer wins!");
    } else if dealer_hand.total_value > BUST_THRESHOLD {
        println!("Dealer busts! You win!");
    } else if player_hand.total_value > dealer_hand.total_value {
        println!("You win!");
    } else {
        println!("Dealer wins!");
    }
}

fn calc_dealer_hand(player_hand: Hand, mut dealer_hand: Hand, deck: &mut Vec<Card>) {
    let dealer_hit_threshold: u8 = 16;

    loop {
        match dealer_hand.total_value.cmp(&dealer_hit_threshold) {
            Ordering::Less => dealer_hand = draw_card(dealer_hand, deck, 1),
            Ordering::Greater => {
                end_game(player_hand, dealer_hand);
                break;
            },
            Ordering::Equal => dealer_hand = draw_card(dealer_hand, deck, 1),
        }
    }
}

fn readable_cards(hand: &Hand) -> String {
    let mut readable_hand: Vec<String> = Vec::new();
    for card in &hand.cards {
        let suit_value_combo = card.value.to_owned() + card.suit;
        readable_hand.push(suit_value_combo);
    }
    let new_hand = readable_hand.join(", ");
    new_hand
}