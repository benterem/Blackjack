use std::io;
use rand::Rng;


struct Hand {
    total: u8,
    num_cards: u8,
    //will not need both fields
    has_ace: bool,
    num_aces: u8,
    cards: Vec<String>
}

fn main() {
    println!("Welcome to blackjack!");
    println!("How many decks would you like to play with?");
    println!("Please enter a number between 6 and 10. The value will default to 6.");

    let mut num_decks = String::new();

    io::stdin()
        .read_line(&mut num_decks)
        .expect("Failed to read line");

    
    let mut num_decks:u8 = match num_decks.trim().parse() {
        Ok(num) => num,
        Err(_) => 6,
    };

    if num_decks < 1 || num_decks > 10 {
        num_decks = 6;
    }
    
    let mut player_hand = Hand {
        total: 0,
        num_cards: 0,
        has_ace: false,
        num_aces: 0,
        cards: Vec::new()
    };
    
    let mut dealer_hand = Hand {
        total: 0,
        num_cards: 0,
        has_ace: false,
        num_aces: 0,
        cards: Vec::new()
    };
    
    //Initial Draw
    println!("");
    for turn in 0..4{
        if turn % 2 == 0{
            draw_card(num_decks, &mut player_hand);
            println!("You drew: {}", player_hand.cards[turn / 2]);
            println!("Your hand total: {}", player_hand.total);
            if player_hand.total == 21 {
                println!("Congradulations!, You've won!")
            };
        }else if turn == 1{
            draw_card(num_decks, &mut dealer_hand);
            println!("Dealer's face up card is: {}", dealer_hand.cards[0]);
        }else {
            draw_card(num_decks, &mut dealer_hand);
        }
        println!("")
    }
    
    let mut hit:bool = true;
    let mut draw_number:usize = 1;
    while hit {
        
        draw_card(num_decks, &mut player_hand);
        draw_number += 1;
        let hand_total = player_hand.total;

        println!("You drew: {}", player_hand.cards[draw_number]);
        
        if hand_total > 21 {
            println!("BUST! your hand total is {}, over 21. You lose!", hand_total);
            break;
        }
        
        if hand_total == 21 {
            println!("21!!!!! YOU WON!!!!");
            break;
        }

        print!("Your hand total: ");
        if player_hand.has_ace {
            println!("soft {}", hand_total);
        }else{
            println!("{}", hand_total)
        }

        println!("Hit or Stand?");

        loop{
            let mut player_decision = String::new();

            io::stdin()
                .read_line(&mut player_decision)
                .expect("Failed to read line");
            
            println!("{}", player_decision);

            match player_decision.to_lowercase().trim() {
                "hit" => {
                    hit = true;
                    println!("will leave");
                    break;
                },
                "stand" => {
                    hit = false;
                    println!("will leave");
                    break;
                },
                _ => {
                    println!("Please enter \"hit\" or \"stand\"");
                    continue
                },
            }
        }
    }
}


fn draw_card(num_decks: u8, hand: & mut Hand) {
    let deck:u8 = rand::thread_rng().gen_range(0..num_decks);
    let suit:u8 = rand::thread_rng().gen_range(0..4);
    let mut rank:u8 = rand::thread_rng().gen_range(1..14);

    let mut card = String::from("");

    match rank {
        1 => {
            hand.num_aces += 1;
            hand.has_ace = true;
            rank = 11;
            card.push_str("Ace");
        },
        11 => {
            rank = 10;
            card.push_str("Prince");
        },
        12 => {
            rank = 10;
            card.push_str("Queen");
        },
        13 => {
            rank = 10;
            card.push_str("King");
        },
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
    
    hand.cards.push(card);
    hand.num_cards += 1;

    //calculate hand total
    // if hand.total + rank > 21 && hand.has_ace && rank == 11{
    //     hand.total = hand.total + rank - 10;
    // }else {
    //     hand.total += rank;
    // }
    hand.total += rank;
}

// fn choose_ace_value {}

// fn dealer_decision(dealer_hand: & mut Hand, player_hand:&Hand) {
    
// }