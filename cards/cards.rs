#[warn(dead_code)]
enum Card{
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    JACK,
    QUEEN,
    KING,
    ACE
}

struct Hand {
    cards: Vec<Card>
}

impl Card {

    //convert card to value
    fn to_uint(&self) -> usize {
        use Card::*;

        match self {
            TWO => 2,
            THREE => 3,    
            FOUR => 4,
            FIVE => 5,
            SIX => 6,
            SEVEN => 7,
            EIGHT => 8,
            NINE => 9,
            JACK|QUEEN|KING => 10,
            ACE => 11
        }
    }
}

impl Hand {
    fn value(&self) -> usize{
        //finds the value of the cards in the players Hand
        let mut res: usize = 0;
        let mut hasAce: bool = false;

        //loop through the cards in hand
        for i in &self.cards {

            //add up values of cards
            res += i.to_uint();

            //find ace in the hand of cards 
            hasAce = if let Card::ACE = i {
                true
            }
            else {
                false
            }
        }
        
        //if the value is over 21 and there is an ace, make that ace have a value of 1
        if res > 21 && hasAce{
            res -= 10;
        }

        res
    }
}

fn main() {
    use Card::*;
    
    let p1:Hand = Hand{cards: vec![ACE, ACE]};

    println!("{}", p1.value());
}
