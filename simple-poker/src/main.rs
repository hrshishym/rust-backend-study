use rand::seq::SliceRandom;

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

    // シャッフル
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);

    // 手札
    let mut hand: Vec<Card> = Vec::new();
    for _ in 0..5 {
        if let Some(card) = deck.pop() {
            hand.push(card);
        }
    }

    // 手札のソート
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    // 手札の表示
    println!("=== Hand ===");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {:}", i + 1, card.suit, card.rank);
    }

    // 入れ替え
    println!("\n=== 手札交換 ===\n");
    println!("交換したいカードの番号を入力してください (例： 1 2 3):");
    // ユーザー入力
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let indices: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap_or(0))
        .collect::<Vec<usize>>();

    for number in indices {
        hand[number - 1] = if let Some(card) = deck.pop() {
            card
        } else {
            // デッキが空の場合、手札のカードをそのまま返す
            hand[number - 1]
        };
    }

    // 手札のソート
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));

    // 手札の表示
    println!("\n=== 新しい手札 ===\n");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {:}", i + 1, card.suit, card.rank);
    }
}
