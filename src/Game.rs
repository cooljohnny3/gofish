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

    fn game_loop(&self) {
        let mut end = true;
        println!("Welcome to gofish!");
        while !end {
            println!("Start a game? (Y/N)");
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(n) => {
                    self.status_menu();
                }
                Err(error) => println!("error: {}", error),
            }

            
        };
    }
}

#[test]
fn test_dealing() {
    let mut g = Game::new();
    let hand = deck::Hand::new_with_cards(g.deal());
    
    assert_eq!(g.deck.get_cards_left(), 47);
    assert_eq!(hand.size(), 5);
}