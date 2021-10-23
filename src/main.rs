use std::io;
use rand::Rng;


fn main() {
    println!("Welcome to blackjack!");
    println!("How many decks would you like to play with?");

    let mut num_decks = String::new();

    io::stdin()
        .read_line(&mut num_decks)
        .expect("Failed to read line");

    draw_card();
    draw_card();
    draw_card();


    //init hashmap

}


fn draw_card() {
    let deck = rand::thread_rng().gen_range(0..6);
    let suit = rand::thread_rng().gen_range(0..4);
    let rank = rand::thread_rng().gen_range(1..14);
}