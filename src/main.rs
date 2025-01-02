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
    
    fn shuffle(&self){

    }
}

fn main() {
    let deck = Deck::new();
    println!("Here's your deck: {:#?}", deck);
}
