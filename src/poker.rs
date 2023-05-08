#[derive(Debug, PartialEq)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq)]
enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

struct Card {
    rank: Rank,
    suite: Suit,
}

struct Hand<'a> {
    cards: Vec<&'a str>,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Combination {
    FlashStreet,
    Four,
    FullHouse,
    Flash,
    Street,
    Triple,
    TwoPairs,
    Pair,
    HigherCard,
}

impl Hand {
    fn combination(&self) -> Combination {
        match &self.hand {
            arr if is => Combination::Flash,
            _ => Combination::HigherCard
        }
    }
    fn ranks(&self) -> Vec<Rank> {
        &self.hand.map(|c| Rank)
    }
    // fn is_flash_street(&self) -> bool {
    //     // &self.hand.
    // }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let hs = hands.map(|hand| Hand { cards: hand });

    vec![]
}
