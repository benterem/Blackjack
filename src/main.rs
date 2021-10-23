use std::io;
use rand::Rng;


fn main() {
    println!("Welcome to blackjack!");
    println!("How many decks would you like to play with?");

    let mut num_decks = String::new();

    io::stdin()
        .read_line(&mut num_decks)
        .expect("Failed to read line");

    let num_decks:u8 = num_decks.trim().parse().expect("Please type a number");
    
    draw_card(num_decks)
}


fn draw_card(num_decks: u8) {
    let deck:u8 = rand::thread_rng().gen_range(0..num_decks);
    let suit:u8 = rand::thread_rng().gen_range(0..=3);
    let rank:u8 = rand::thread_rng().gen_range(1..14);

    println!("{} {} {}", rank, suit, deck);

    let mut card = String::from("");

    match rank {
        1 => card.push_str("Ace"),
        11 => card.push_str("Prince"),
        12 => card.push_str("Queen"),
        13 => card.push_str("King"),
        other => card.push_str(&other.to_string())
    }
    
    
    match suit {
        0 => card.push_str(" of Hearts"),
        1 => card.push_str(" of Spades"),
        2 => card.push_str(" of Diamonds"),
        _ => card.push_str(" of Clubs"),
    }

    card.push_str(&deck.to_string());
    
    println!("{}", card);
    // cards_drawn.insert(card, true);
}