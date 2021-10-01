use rand::{thread_rng, seq::{SliceRandom, IteratorRandom}};
use std::cmp::Ordering;

mod input {
    use std::io;

    pub fn int(msg: &str) -> usize {
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

#[derive(Copy, Clone)]
pub enum Card {
    None,
    Clubs(u8),
    Hearts(u8),
    Spades(u8),
    Diamonds(u8)
}
impl Card {
    pub const ACE: u8 = 1;
    pub const KING: u8 = 13;

    fn suit_sym(&self) -> char {
        match self {
            Card::None => ' ',
            Card::Clubs(_) => '♣',
            Card::Hearts(_) => '♥',
            Card::Spades(_) => '♠',
            Card::Diamonds(_) => '♦'
        }
    }

    fn rank_sym(&self) -> String {
        format!("{}", match self.num() {
            Some(1) => "A".to_string(),
            Some(11) => "J".to_string(),
            Some(12) => "Q".to_string(),
            Some(13) => "K".to_string(),
            Some(num) => num.to_string(),
            None => "".to_string(),
        })
    }

    fn draw(&self) -> [String; 5] {
        [
            String::from("┌─────┐"),
            format!("│{:<2}   │", self.rank_sym()),
            format!("│{0:<2} {0:>2}│", self.suit_sym()),
            format!("│   {:>2}│", self.rank_sym()),
            String::from("└─────┘")
        ]
    }

    pub fn to_none(&mut self) {
        *self = Card::None
    }

    pub fn num(&self) -> Option<u8> {
        match self {
            Card::Clubs(num)
            | Card::Hearts(num)
            | Card::Spades(num)
            | Card::Diamonds(num) => Some(*num),
            Card::None => None
        }
    }

    pub fn show_vec_cards(cards: &Vec<Card>) {
        let mut output: [String; 5] = Default::default();
        for card in cards {
            for (line_card, line_out) in (*card).draw().iter().zip(&mut output) {
                (*line_out).push_str(&line_card);
            }
        }
        for line in output {
            println!("{}", line);
        }
    }

    pub fn show(&self) {
        for line in self.draw() {
            println!("{}", line);
        }
    }

    pub fn new_prize_pool() -> Vec<Card> {
        let mut rng = thread_rng();
        let mut pool: Vec<Card> = (Card::ACE..=Card::KING).map(|x| Card::Diamonds(x)).collect();
        pool.shuffle(&mut rng);
        pool
    }
}

struct Player {
    hand: Vec<Card>,
    pool: Vec<Card>
}
impl Player {
    pub fn new() -> Player {
        Player {
            hand: (Card::ACE..=Card::KING).map(|x| Card::Hearts(x)).collect(),
            pool: Vec::new()
        }
    }
    pub fn new_computer() -> Player {
        Player {
            hand: (Card::ACE..=Card::KING).map(|x| Card::Spades(x)).collect(),
            pool: Vec::new()
        }
    }

    fn is_valid_choice(&self, spot: usize) -> bool {
        if let Some(card) = self.hand.get(spot) {
            return !matches!(card, Card::None);
        }
        false
    }

    pub fn get_choice(&mut self) -> usize {
        loop {
            let choice = input::int("Bid a card by number (1-13): ");
            if choice != 0 && self.is_valid_choice(choice - 1) {
                (*self).hand[choice - 1].to_none();
                return choice;
            }
        }
    }

    pub fn get_rand(&mut self) -> usize {
        let mut rng = thread_rng();
        let choices = self.hand
                        .iter_mut()
                        .enumerate()
                        .filter(|(_, card)| !matches!{ card, Card::None });
        let (index, card) = choices.choose(&mut rng).expect("Computer somehow ran out of cards.");
        card.to_none();
        index + 1
    }

    fn accumulate(&self) -> u8 {
        let mut total: u8 = 0;
        for card in &self.pool {
            total += card.num().unwrap_or(0);
        }
        total
    }
}

fn main() {
    let mut discarded: Vec<Card> = Vec::new();
    let mut prizes = Card::new_prize_pool();
    let mut computer = Player::new_computer();
    let mut player = Player::new();
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
        let computer_choice = computer.get_rand();
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

    let computer_total = computer.accumulate();
    let player_total = player.accumulate();

    println!("computer total : {}", computer_total);
    println!("your total     : {}", player_total);
    println!("          ");
    match player_total.cmp(&computer_total) {
        Ordering::Greater => println!("{:^10}", "You win!!"),
        Ordering::Less => println!("{:^10}", "You lose."),
        Ordering::Equal => println!("{:^10}", "Draw!"),
    }
    println!("############################################\n");

    [&computer.pool, &player.pool]
        .iter()
        .zip(["computer's pool", "your pool"])
        .for_each(|(pool, name)| {
            println!("{}", name); Card::show_vec_cards(pool);
        });

    if discarded.len() > 0 {
        println!("Discarded cards: ");
        Card::show_vec_cards(&discarded);
    } else {
        println!("No cards were discarded.");
    }
}
