use rand::thread_rng; 
use rand::seq::SliceRandom;

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {

        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    //agregamos el concepto de borrowing en el cual self toma prestado la propiedad de deck 
    // deal toma un prestamo mutable de deck por lo tanto en main al llamar a deck seguida 
    // siendo valido.

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(
            self.cards.len() - num_cards
        )
    }
    
}
fn main() {
    let mut deck: Deck = Deck::new();

    deck.shuffle();
    let cards = deck.deal(3);


    println!("Heres your hand: {:#?}", cards);
    println!("Heres your deck: {:#?}", deck);

}
