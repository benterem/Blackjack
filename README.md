# Blackjack

After getting this assignment and my conversation with Jon, I decided to write the game in Rust. The decision was twofold:

a) Rust is one of the languages in your tech stack

b) I am currently learning it, and I wanted to push my self

Rust is a general-purpose programming language (and a great one at that!). There is no special instruction, other than to build with cargo.

The game is played on the console and the gameplay is pretty simple:

## The player enters the number of ecks

The player can enter the number of decks for the game, it will default to 6 and the possible range is 6 - 9 (inclusive)

## Initial Draw

The player and dealer are dealt and notified of their cards. Dealer's second card is left "upside down".

## Player's turn

The player's cards are displayed with their total. They are then prompted to "hit" or "stand". They must type one of these options and will be prompted to continuously until they do. Their turn ends if:

1) They enter "stand"

2) Go over 21

3) Get 21

## Dealer's turn

If the player doesn't bust, the dealer will start their turn. There are strictly defined rules for the dealer's play. 

If their hand is larger than 17, they must stand. Otherwise they must hit. This goes on until the dealer busts or gets a hand total of 17 or larger.


## Determining the winner

The winner is determined as follows:

1) If the player busts, the dealer wins

2) If the player gets 21, the winner depends on the dealer. If the dealer got 21 its a draw. Otherwise, the player wins.

3) If the dealer busts or the winner gets higher than the dealer, the player wins.

4) If the dealer gets higher, they win. 

5) Otherwise its a draw.

At this point the player is prompted to either play another round, by typing play or exit by typing anything else.
