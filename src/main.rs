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
    // let mut hit:bool = true;
    let mut player_hand: Vec<String> = Vec::new();
    let mut deal_hand: Vec<String> = Vec::new();
    let mut player_total = 0;
    let mut dealer_total = 0;
    
    for turn in 0..4{
        if turn % 2 == 0{
            player_total += draw_card(num_decks, &mut player_hand);
            println!("You drew {}", player_hand[turn / 2]);
            println!("Your hand total: {}", player_total)
        }else {
            dealer_total += draw_card(num_decks, &mut deal_hand);
            println!("Dealer drew {}", deal_hand[turn / 2]);
            println!("Your hand total: {}", dealer_total)
        }
        println!("")
    }

    
}


fn draw_card(num_decks: u8, hand: &mut Vec<String>) -> u8 {
    let deck:u8 = rand::thread_rng().gen_range(0..num_decks);
    let suit:u8 = rand::thread_rng().gen_range(0..4);
    let rank:u8 = rand::thread_rng().gen_range(1..14);

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

    let mut card_with_deck_number = card.clone();
    card_with_deck_number.push_str(&deck.to_string());
    
    println!("{}", card);

    hand.push(card);
    if rank > 10 {
        return 10;
    }
    rank
}