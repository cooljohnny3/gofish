use std::fmt;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
enum Suit {
    Club,
    Spade,
    Heart,
    Diamond
}

const SUITS: [Suit; 4] = [Suit::Club, Suit::Spade, Suit::Heart, Suit::Diamond];

impl Suit {
    pub fn to_char(self) -> char {
        match self {
            Self::Diamond => '♦',
            Self::Spade => '♠',
            Self::Heart => '♥',
            Self::Club => '♣',
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
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
    Ten,
    Jack,
    Queen,
    King
}

const RANKS: [Rank; 13] = [
    Rank::Ace,
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King
];

impl Rank {
    pub fn to_char(self) -> char {
        match self {
            Self::Ace => 'A',
            Self::King => 'K',
            Self::Queen => 'Q',
            Self::Jack => 'J',
            Self::Ten => 'T',
            Self::Nine => '9',
            Self::Eight => '8',
            Self::Seven => '7',
            Self::Six => '6',
            Self::Five => '5',
            Self::Four => '4',
            Self::Three => '3',
            Self::Two => '2',
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
pub struct Card {
    suit: Suit,
    rank: Rank
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{} ", self.rank.to_char(), self.suit.to_char())
    }
}

pub struct Hand {
    cards: Vec<Card>
}

impl Hand {
    pub fn new() -> Self {
        Self { cards: vec![] }
    }

    pub fn new_with_cards(cards: Vec<Card>) -> Self {
        Self { cards: cards }
    }

    pub fn add(&mut self, c: Card) {
        self.cards.push(c)
    }

    pub fn remove(&mut self, i: usize) -> Card {
        self.cards.remove(i)
    }

    pub fn contains(&self, c: Card) -> bool {
        self.cards.contains(&c)
    }

    pub fn size(&self) -> usize {
        self.cards.len()
    }

    pub fn ittr(&self) -> std::slice::Iter<'_, Card> {
        self.cards.iter()
    }
}

pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    // Creates a vector with all 52 combinations of suits and ranks and returns the Deck
    pub fn new() -> Self {
        let mut temp = vec![];
        for s in &SUITS {
            for r in &RANKS {
                temp.push(Card {
                    suit: *s,
                    rank: *r
                });
            }
        };
        let mut rng = thread_rng();
        temp.shuffle(&mut rng);
        Self { cards: temp }
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn get_cards_left(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}