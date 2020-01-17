use std::io;

mod deck;

struct Player {
    pairs: i32,
    hand: deck::Hand
}

impl Player {
    pub fn new() -> Self {
        Self { pairs: 0, hand: deck::Hand::new() }
    }

    pub fn get_cards(&self) -> &deck::Hand {
        &self.hand
    }
}

pub struct Game {
    deck: deck::Deck,
    player: Player,
    opponent: Player,
    turn: usize
}

impl Game {
    pub fn new() -> Self {
        Self {
            deck: deck::Deck::new(),
            player: Player::new(),
            opponent: Player::new(),
            turn: 1
        }
    }

    pub fn start(&mut self) {
        self.player = Player { pairs: 0, hand: deck::Hand::new_with_cards(self.deal()) };
        self.opponent = Player { pairs: 0, hand: deck::Hand::new_with_cards(self.deal()) };

        self.game_loop();
    }

    fn deal(&mut self) -> Vec<deck::Card> {
        let mut cards = vec![];
        for _i in 0..5 {
            // pop card from top of deck
            let temp = self.deck.pop().unwrap();
            // Add card to hand
            cards.push(temp);
        }
        cards
    }

    fn status_menu(&self) {
        // Display your cards, # of opponent's cards both player's # of pairs
        // Turn number, cards left in deck
        println!("Turn: {} Cards Left {}", self.turn, self.deck.get_cards_left());
        println!("Score: Player: {} Computer: {}", self.player.pairs, self.opponent.pairs);
        println!("Opponent has {} cards in their hand.", self.opponent.hand.size());
        print!("Your cards: ");
        for card in self.player.hand.ittr() {
            print!("{}", card);
        }
        println!("");
    }

    fn game_loop(&mut self) {
        let mut end = false;
        println!("Welcome to gofish!");
        while !end {
            println!("Start a game? (Y/N) ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Invalid input");
            let trimmed_input = input.trim();
            if trimmed_input == "Y" || trimmed_input == "y" {
                self.status_menu();
                // check pairs
                // play pairs
                self.check_pairs(self.player.get_cards());

                // show status ahgain if pairs were found
                // give selection of card to call
                // check opponent if selected card is present in hand
                // get card or go fish (draw)
                end = true;
            } else if trimmed_input == "N" || trimmed_input == "n" {
                end = true;
            } else {
                println!("Please enter Y or N");
            }
        };
    }

    // Checks for pairs in hand
    // returns the cards to remove
    fn check_pairs(&self, h: &deck::Hand) -> Vec<deck::Card> {
        let hand = h.get_cards();
        let mut cards = vec![];
        for i in 0..hand.len() {
            for j in i..hand.len() {
                if hand[i].rank == hand[j].rank && !cards.contains(&hand[i]) && !cards.contains(&hand[j]) {
                    cards.push(hand[i]);
                    cards.push(hand[j]);
                }
            }
        };
        cards
    }
}

#[test]
fn test_dealing() {
    let mut g = Game::new();
    let hand = deck::Hand::new_with_cards(g.deal());
    
    assert_eq!(g.deck.get_cards_left(), 47);
    assert_eq!(hand.size(), 5);
}

#[test]
fn test_check_pairs() {
    let g = Game::new();
    let hand = deck::Hand::new_with_cards(vec![
        deck::Card { rank: deck::Rank::Five, suit: deck::Suit::Diamond },
        deck::Card { rank: deck::Rank::Five, suit: deck::Suit::Club },
        deck::Card { rank: deck::Rank::Three, suit: deck::Suit::Diamond },
        deck::Card { rank: deck::Rank::Five, suit: deck::Suit::Heart },
        deck::Card { rank: deck::Rank::Three, suit: deck::Suit::Club },
    ]);
    assert_eq!(g.check_pairs(&hand), vec![
        deck::Card { rank: deck::Rank::Five, suit: deck::Suit::Diamond },
        deck::Card { rank: deck::Rank::Five, suit: deck::Suit::Club },
        deck::Card { rank: deck::Rank::Five, suit: deck::Suit::Heart },
        deck::Card { rank: deck::Rank::Three, suit: deck::Suit::Club },
    ]);
}