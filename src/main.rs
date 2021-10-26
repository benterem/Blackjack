use std::io;
use rand::Rng;
use std::collections::HashMap;

struct Hand {
    total: u16,
    num_cards: u8,
    has_ace: bool,
    ace_reduced: bool,
    cards: Vec<String>
}

fn main() {
    println!("Welcome to blackjack!");
    println!("How many decks would you like to play with?");
    println!("Please enter a number between 6 and 10. The value will default to 6.");

    //number of decks in game
    let mut num_decks = String::new();

    io::stdin()
        .read_line(&mut num_decks)
        .expect("Failed to read line");

    //default 
    let mut num_decks:u16 = match num_decks.trim().parse() {
        Ok(num) => num,
        Err(_) => 6
    };

    if num_decks < 1 || num_decks > 10 {
        num_decks = 6;
    }
    
    
    //Start of game
    loop{        
        let mut player_hand = Hand {
            total: 0,
            num_cards: 0,
            has_ace: false,
            ace_reduced: false,
            cards: Vec::new()
        };
        
        let mut dealer_hand = Hand {
            total: 0,
            num_cards: 0,
            has_ace: false,
            ace_reduced: false,
            cards: Vec::new()
        };
        
        let mut cards_drawn:HashMap<u16, bool> = HashMap::new();
        
        //original draw
        for turn in 0..4{
            if turn % 2 == 0{
                let card = draw_card(& mut cards_drawn, num_decks, &mut player_hand);
                println!("\nYou drew: {}", card);
            }else{
                let card =draw_card(& mut cards_drawn, num_decks, &mut dealer_hand);
                if turn == 1 {
                    println!("\nDealer's face up card is: {}", card);
                }
            }
        }
                
        //player's turn
        loop {  
            let hand_total = player_hand.total;
            
            println!("\nYour hand:");
            for card in &player_hand.cards {
                println!("{}", card);
            }
            
            print!("\nYour hand total: ");
            if hand_total < 21 && player_hand.has_ace && !player_hand.ace_reduced {
                println!("soft {}", hand_total);
            }else{
                println!("{}", hand_total)
            }

            if hand_total > 21 {
                println!("\nBUST!");
                break;
            }
            
            if hand_total == 21 {

                break;
            }

            println!("\nHit or Stand?");

            let mut player_decision = String::new();

            io::stdin()
                .read_line(&mut player_decision)
                .expect("Failed to read line");

            match player_decision.to_lowercase().trim() {
                "hit" => {},
                "stand" => {
                    break;
                },
                _ => {
                    println!("Please enter \"hit\" or \"stand\"");
                    continue
                },
            }

            let card = draw_card(& mut cards_drawn, num_decks, &mut player_hand);
            println!("\nYou drew: {}", card);
        }


        //dealer's play
        println!("\nDealer flips face down card");

        if player_hand.total > 21 {
            println!("Dealer's hand:");
            for card in &dealer_hand.cards{
                println!("{}", card);
            }
            println!("\nDealer's total: {}", &dealer_hand.total);
        }else{    
            loop {

                let dealer_total = dealer_hand.total;

                println!("\nDealer's hand:");
                for card in &dealer_hand.cards{
                    println!("{}", card);
                }
                println!("\nDealer's total: {}", dealer_total);
                
                if dealer_total < 17 {
                    let card = draw_card(& mut cards_drawn, num_decks, & mut dealer_hand);
                    println!("\nDealer drew: {}", card);
                }else if dealer_total < 21 {
                    break;
                }else if dealer_total == 21 {
                    println!("\nDealer got 21");
                    break;
                }else {
                    println!("\nDEALER BUST!");
                    break;
                }
            }
        }

        // determening a winner
        let player = player_hand.total;
        let dealer=dealer_hand.total;

        if player > 21 {
            println!("\nDealer wins.");
        }else if player == 21 {
            if dealer == 21 {
                println!("\nDraw.");
                break;
            }
            println!("\n21 YOU WIN!!!!!");
        }else if dealer > 21 || player > dealer {
            println!("\nYOU WIN!!!!!");
        }else if dealer > player {
            println!("\nDealer wins.");
        }else {
            println!("\nDraw.")
        }

        println!("\nAnother round?");
        println!("Enter \"yes\" to continue");

        let mut play_again = String::new();

        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        match play_again.to_lowercase().trim() {
            "yes" => {
                println!("\nAwesome! Dealing the cards!");
            },
            _ => {
                println!("\nGood-bye!");
                break;
            }
        }
    }
}


fn draw_card(cards_drawn: & mut HashMap<u16, bool>,num_decks: u16, hand: & mut Hand) -> String {
    loop {    
        let deck:u16 = rand::thread_rng().gen_range(1..=num_decks);
        let suit:u16 = rand::thread_rng().gen_range(1..5);
        let mut rank:u16 = rand::thread_rng().gen_range(1..14);
        let mut ace:bool = false;
        let mut card = String::from("");

        //check that card has now been previously drawn
        let card_id:u16 = 100*rank + 10*suit + deck;

        match cards_drawn.contains_key(&card_id) {
            true => continue,
            _ => {
                cards_drawn.insert(card_id, true);
            }
        }

        match rank {
            1 => {
                ace = true;
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
            1 => card.push_str(" of Hearts"),
            2 => card.push_str(" of Spades"),
            3 => card.push_str(" of Diamonds"),
            _ => card.push_str(" of Clubs"),
        }

        hand.cards.push(card.clone());
        hand.num_cards += 1;

        //calculate hand total

        //if the drawn card is an ace, and the new total is > 21, ace counts as 1 
        if hand.total + rank > 21 && ace {
            hand.total = hand.total + 1;
            hand.ace_reduced = true;
        //if the drawn card has rank >= 10, and player has an ace, reduce ace to 1
        }else if hand.total + rank > 21 && !hand.ace_reduced && hand.has_ace{
            hand.total = hand.total + rank - 10;
            hand.ace_reduced = true
        }else {
            hand.total += rank;
        }
        return card;
    }
}