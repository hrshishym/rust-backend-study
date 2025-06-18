use rand::seq::SliceRandom;

// 
#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

//
#[derive(Debug, Clone, Copy, PartialEq)]
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

    // 役の判定
    // フラッシュ
    let suit = hand.first().unwrap().suit;
    let is_flash = hand.iter().all(|card| card.suit == suit);

    // ペア数のチェック
    // 1から13までのランクの出現回数をカウント
    let mut rank_count = [0; 14]; // 1から13までのランクをカウントするための配列
    for card in &hand {
        rank_count[card.rank as usize] += 1;
    }

    if is_flash {
        println!("\n=== フラッシュ ===");
    } else if rank_count.iter().any(|&count| count == 4) {
        println!("\n=== フォーカード ===");
    } else if rank_count.iter().any(|&count| count == 3) && rank_count.iter().any(|&count| count == 2) {
        println!("\n=== フルハウス ===");
    } else if rank_count.iter().any(|&count| count == 3) {
        println!("\n=== スリーカード ===");
    } else if rank_count.iter().filter(|&&count| count == 2).count() == 2 {
        println!("\n=== ツーペア ===");
    } else if rank_count.iter().any(|&count| count == 2) {
        println!("\n=== ワンペア ===");
    } else {
        println!("\n=== ノーペア ===");
    }
}
