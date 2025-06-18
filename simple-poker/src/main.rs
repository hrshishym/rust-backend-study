// 
#[derive(Debug)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

//
#[derive(Debug)]
struct Card {
    suit: Suit,
    rank: i32,
}

fn main() {
    let suit = Suit::Club;
    let rank = 10;
    let card = Card { suit, rank };
    println!("Created a card with suit: {:?} and rank: {}", card.suit, card.rank);
    println!("Card is {:?}", card);
}
