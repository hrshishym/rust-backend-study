// 
#[derive(Debug, Clone, Copy)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

//
#[derive(Debug, Clone, Copy)]
struct Card {
    suit: Suit,
    rank: i32,
}

fn main() {
    // Vec
    let mut deck: Vec<Card> = Vec::new();
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];
    
    // Deckの作成
    for suit in suits {
        for rank in 1..=13 {
            deck.push(Card { suit, rank });
        }
    }

    println!("Deck: {:?}", deck);

}
