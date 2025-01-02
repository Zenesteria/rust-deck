#[derive(Debug)]
struct Deck{
    cards:Vec<String>
}

fn main() {
    // List of 'suits' - 'hearts', 'spades'
    let suits = ["Hearts", "Spades", "Diamonds"];
    // List of 'values' - 'ace', 'two', 'three'
    let values = ["Ace", "Two", "Three"];

    let cards = vec![];

    // Double nested loop
    for suit in suits{
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(value);
        }
    }


    let deck = Deck{cards:vec![]};
    println!("Here's your deck: {:?}", deck);
}
