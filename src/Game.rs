use std::io;

mod deck;

struct Player {
    pairs: usize,
    hand: deck::Hand
}

impl Player {
    pub fn new() -> Self {
        Self { pairs: 0, hand: deck::Hand::new() }
    }

    pub fn get_cards(&self) -> &deck::Hand {
        &self.hand
    }

    pub fn set_cards(&mut self, new_hand: deck::Hand) {
        self.hand = new_hand;
    }

    pub fn add_pairs(&mut self, pairs: usize) {
        self.pairs += pairs;
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
        println!("Computer has {} cards in their hand.", self.opponent.hand.size());
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
                // check pairs for player
                let pairs = self.check_pairs(self.player.get_cards());
                // check if pairs were found
                if pairs.get_cards().len() != self.player.get_cards().get_cards().len() {
                    // add points
                    self.player.add_pairs((self.player.get_cards().get_cards().len() - pairs.get_cards().len()) / 2);
                    // set the new hand
                    self.player.set_cards(pairs);
                    // show status again with new points and hand
                    self.status_menu();
                }
                // check pairs for opponent
                let pairs = self.check_pairs(self.opponent.get_cards());
                // check if pairs were found
                if pairs.get_cards().len() != self.opponent.get_cards().get_cards().len() {
                    // add points
                    self.opponent.add_pairs((self.opponent.get_cards().get_cards().len() - pairs.get_cards().len()) / 2);
                    // set the new hand
                    self.opponent.set_cards(pairs);
                    // show status again with new points and hand
                    self.status_menu();
                }
                // give selection of card to call
                let card = self.pick_card();
                // check opponent if selected card is present in hand
                let option = self.check_selection(&card, &self.opponent);
                // get card or go fish (draw)
                // opponent turn
                end = true;
            } else if trimmed_input == "N" || trimmed_input == "n" {
                end = true;
            } else {
                println!("Please enter Y or N");
            }
        };
    }

    // Checks for pairs in hand
    // returns new Hand without pairs
    fn check_pairs(&self, h: &deck::Hand) -> deck::Hand {
        let hand = h.get_cards();
        let mut cards = vec![];
        for i in 0..hand.len() {
            for j in i..hand.len() {
                if i != j && hand[i].rank == hand[j].rank && 
                !cards.contains(&hand[i]) && !cards.contains(&hand[j]) {
                    println!("Pair of {}s found", hand[i].rank.to_char());
                    cards.push(hand[i]);
                    cards.push(hand[j]);
                }
            }
        };
        deck::Hand::new_with_cards(cards)
    }

    // pick a card from the player's hand
    fn pick_card(&self) -> deck::Card {
        println!("Which card do you want to call?");
        let cards = self.player.get_cards().get_cards();
        for i in 0..cards.len() {
            println!("{}. {}", i, cards[i]);
        }
        let mut input;
        loop {
            let mut raw_input = String::new();
            io::stdin().read_line(&mut raw_input).expect("Invalid input");
            let trimmed_input = raw_input.trim();
            input = trimmed_input.parse::<usize>().expect("Please enter a positive number");
            if input < cards.len() {
                break
            }
        } 
        cards[input]
    }

    fn check_selection(&self, c: &deck::Card, p: &Player) -> Option<deck::Card> {
        let mut value = None;
        let cards = p.get_cards().get_cards();
        for i in cards {
            if i.rank == c.rank {
                value = Some(*i)
            }
        }
        return value
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_dealing() {
        let mut g = Game::new();
        let hand = deck::Hand::new_with_cards(g.deal());
        
        assert_eq!(g.deck.get_cards_left(), 47);
        assert_eq!(hand.size(), 5);
    }

    #[test]
    fn test_check_pairs1() {
        let g = Game::new();
        let hand = deck::Hand::new_with_cards(vec![
            deck::Card { rank: deck::Rank::Five, suit: deck::Suit::Diamond },
            deck::Card { rank: deck::Rank::Five, suit: deck::Suit::Club },
            deck::Card { rank: deck::Rank::Three, suit: deck::Suit::Diamond },
            deck::Card { rank: deck::Rank::Two, suit: deck::Suit::Heart },
            deck::Card { rank: deck::Rank::Three, suit: deck::Suit::Club },
        ]);
        assert_eq!(*g.check_pairs(&hand).get_cards(), vec![
            deck::Card { rank: deck::Rank::Five, suit: deck::Suit::Diamond },
            deck::Card { rank: deck::Rank::Five, suit: deck::Suit::Club },
            deck::Card { rank: deck::Rank::Three, suit: deck::Suit::Diamond },
            deck::Card { rank: deck::Rank::Three, suit: deck::Suit::Club },
        ]);
    }

    #[test]
    fn test_check_selection() {
        let mut g = Game::new();
        let hand1 = deck::Hand::new_with_cards(g.deal());
        let hand2 = deck::Hand::new_with_cards(g.deal());
        g.player.set_cards(hand1);
        g.opponent.set_cards(hand2);

        for card in g.player.hand.get_cards() {
            let selection = g.check_selection(card, &g.opponent);
            
        }
    }
}