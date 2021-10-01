use rand::{thread_rng, seq::SliceRandom};
use std::cmp::Ordering;

mod input {
    use std::io;

    pub fn uint(msg: &str) -> usize {
        loop {
            println!("{}", msg);
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            match input.trim().parse::<usize>() {
                Ok(num) => return num,
                Err(_) => {
                    println!("\nFailed to parse number\n");
                    continue;
                }
            };
        }
    }
}

enum Suit {
    None, Clubs, Hearts, Spades, Diamonds
}
impl Suit {
    fn make(&self, rank: u8) -> Card {
        match self {
            Suit::None => Card::None,
            Suit::Clubs => Card::Clubs(rank),
            Suit::Hearts => Card::Hearts(rank),
            Suit::Spades => Card::Spades(rank),
            Suit::Diamonds => Card::Diamonds(rank),
        }
    }
}

#[derive(Copy, Clone)]
enum Card {
    None,
    Clubs(u8),
    Hearts(u8),
    Spades(u8),
    Diamonds(u8)
}
impl Card {
    const ACE: u8 = 1;
    const KING: u8 = 13;

    fn _suit_sym(&self) -> char {
        match self {
            Card::None => ' ',
            Card::Clubs(_) => '♣',
            Card::Hearts(_) => '♥',
            Card::Spades(_) => '♠',
            Card::Diamonds(_) => '♦'
        }
    }

    fn _rank_sym(&self) -> String {
        format!("{}", match self.num() {
            Some(1) => "A".to_string(),
            Some(11) => "J".to_string(),
            Some(12) => "Q".to_string(),
            Some(13) => "K".to_string(),
            Some(num) => num.to_string(),
            None => "".to_string(),
        })
    }

    fn _draw(&self) -> [String; 5] {
        [
            String::from("┌─────┐"),
            format!("│{:<2}   │", self._rank_sym()),
            format!("│{0:<2} {0:>2}│", self._suit_sym()),
            format!("│   {:>2}│", self._rank_sym()),
            String::from("└─────┘")
        ]
    }

    fn to_none(&mut self) {
        *self = Card::None
    }

    fn num(&self) -> Option<u8> {
        match self {
            Card::Clubs(num)
            | Card::Hearts(num)
            | Card::Spades(num)
            | Card::Diamonds(num) => Some(*num),
            Card::None => None
        }
    }

    fn show_vec_cards(cards: &Vec<Card>) {
        let mut output: [String; 5] = Default::default();
        for card in cards {
            for (line_card, line_out) in (*card)._draw().iter().zip(&mut output) {
                (*line_out).push_str(&line_card);
            }
        }
        for line in output {
            println!("{}", line);
        }
    }

    fn show(&self) {
        for line in self._draw() {
            println!("{}", line);
        }
    }

    fn new_set(suit: Suit, random: bool) -> Vec<Card> {
        let mut set: Vec<_> = (Card::ACE..=Card::KING).map(|rank| suit.make(rank)).collect();
        if random {
            let mut rng = thread_rng();
            set.shuffle(&mut rng);
        }
        set
    }
}

struct Player {
    hand: Vec<Card>,
    pool: Vec<Card>
}
impl Player {
    fn new(suit: Suit, random: bool) -> Player {
        Player {
            hand: Card::new_set(suit, random),
            pool: Vec::new()
        }
    }

    fn is_valid_choice(&self, spot: usize) -> bool {
        if let Some(card) = self.hand.get(spot) {
            return !matches!(card, Card::None);
        }
        false
    }

    fn get_choice(&mut self) -> usize {
        loop {
            let choice = input::uint("Bid a card by number (1-13): ");
            if choice != 0 && self.is_valid_choice(choice - 1) {
                (*self).hand[choice - 1].to_none();
                return choice;
            }
        }
    }

    fn pool_sum(&self) -> u8 {
        self.pool
            .iter()
            .map(|card| card.num().unwrap_or(0))
            .sum()
    }
}

fn main() {
    let mut discarded = Vec::<Card>::new();
    let mut prizes = Card::new_set(Suit::Diamonds, true);
    let mut computer = Player::new(Suit::Spades, true);
    let mut player = Player::new(Suit::Hearts, false);
    let mut show_pool = false;

    for prize in prizes.drain(..) {
        println!("The prize: ");
        prize.show();
        println!("Your hand: ");
        Card::show_vec_cards(&player.hand);
        println!("Your pool so far:");
        if show_pool {
            Card::show_vec_cards(&player.pool);
        }

        let player_choice = player.get_choice();
        let computer_choice = computer.hand.pop().unwrap().num().unwrap() as usize;
        println!("--------------------------------------------");
        println!("computer bid : {}", computer_choice);
        println!("you bid      : {}", player_choice);

        match player_choice.cmp(&computer_choice) {
            Ordering::Greater => {
                player.pool.push(prize);
                println!("\nYou won the prize!");
                show_pool = true;
            },
            Ordering::Less => {
                computer.pool.push(prize);
                println!("\nComputer won the prize!");
            },
            Ordering::Equal => {
                discarded.push(prize);
                println!("\nDraw! The prize was discarded");
            }
        }
        println!("--------------------------------------------");
    }

    println!("\n############################################");

    let computer_total = computer.pool_sum();
    let player_total = player.pool_sum();

    println!("computer total : {}", computer_total);
    println!("your total     : {}\n", player_total);

    match player_total.cmp(&computer_total) {
        Ordering::Greater => println!("You win!!"),
        Ordering::Less => println!("You lose."),
        Ordering::Equal => println!("Draw!"),
    }
    println!("############################################\n");

    println!("computer's pool");
    Card::show_vec_cards(&computer.pool);
    println!("your pool");
    Card::show_vec_cards(&player.pool);

    if discarded.len() > 0 {
        println!("Discarded cards: ");
        Card::show_vec_cards(&discarded);
    } else {
        println!("No cards were discarded.");
    }
}
