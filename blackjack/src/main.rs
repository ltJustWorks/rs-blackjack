enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

struct Card {
    rank: Rank,
    suit: Suit,
    value: i32,
}

fn card_value(card: &Card, cards_total: i32) -> i32 {
    match card.rank {
        Rank::Ace => {
            if cards_total + 11 <= 21 {11} else {1}
        }
        Rank::Two..=Rank::Nine => rank as i32,
        Rank::Ten..=Rank::King => 10,
    }
}

fn main() {
    let mut deck: Vec<Card> = Vec::new();

    println!("Blackjack");
}
