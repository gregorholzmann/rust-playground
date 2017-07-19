extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to 21!");

    let dealer_hand = rand::thread_rng().gen_range(2, 21);
    let mut player_hand = rand::thread_rng().gen_range(2, 21);
    let bust_threshold: i32 = 21; 


    loop {
        println!("Your hand is: {}", player_hand);
        println!("Dealer hand is: {}", dealer_hand);

        if player_hand > bust_threshold || player_hand == bust_threshold {
            end_game(player_hand, dealer_hand, bust_threshold);
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
            player_hand = draw_card(&player_hand);
        } else {
            calc_dealer_hand(player_hand, dealer_hand, bust_threshold);
            break;
        }
    }
}

fn draw_card(hand: &i32) -> i32 {
    let inc: i32 = rand::thread_rng().gen_range(1, 11);
    hand + inc
}

fn end_game(player_hand: i32, dealer_hand: i32, bust_threshold: i32) {
    println!("Final Player Hand: {}", player_hand);
    println!("Final Dealer Hand: {}", dealer_hand);
    if player_hand > bust_threshold {
        println!("You bust! Dealer wins!");
    } else if dealer_hand > bust_threshold {
        println!("Dealer busts! You win!");
    } else if player_hand > dealer_hand {
        println!("You win!");
    } else {
        println!("Dealer wins!");
    }

}

fn calc_dealer_hand(player_hand: i32, mut dealer_hand: i32, bust_threshold: i32) {
    let dealer_hit_threshold: i32 = 16;

    loop {
        match dealer_hand.cmp(&dealer_hit_threshold) {
            Ordering::Less => dealer_hand = draw_card(&dealer_hand),
            Ordering::Greater => {
                end_game(player_hand, dealer_hand, bust_threshold);
                break;
            },
            Ordering::Equal => dealer_hand = draw_card(&dealer_hand),
        }
    }
}