use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck{
    cards:Vec<String>
}

// Inherent Implementation
impl Deck {
    fn new() -> Self {
        // List of 'suits' - 'hearts', 'spades'
        let suits = ["Hearts", "Spades", "Diamonds"];
        // List of 'values' - 'ace', 'two', 'three'
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];

        // Double nested loop
        for suit in suits{
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck{cards}
    }
    
    fn shuffle_cards(&mut self){
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle_cards();
    println!("Here's your deck: {:#?}", deck);
}
