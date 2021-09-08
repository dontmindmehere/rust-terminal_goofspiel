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
    Joker,
    Clubs(u8),
    Hearts(u8),
    Spades(u8),
    Diamonds(u8)
}

impl Card {
    pub const ACE: u8 = 1;
    pub const KING: u8 = 13;

    pub fn new(card: Card) -> Result<Card, Card> {
        match card.num() {
            Some(Card::ACE..=Card::KING) | None => Ok(card),
            Some(_) => Err(Card::None)
        }
    }

    pub fn from_byte(byte: u8) -> Card {
        /*
         * 1xxx_xxxx => None,
         * 01xx_xxxx => Joker,
         * 00(rank)(suit) => Regular
        */
        match byte >> 6 {
            0b11 | 0b10 => Card::None,
            0b01 => Card::Joker,
            0b00 => {
                let rank = (byte & 0b_0000_1111) + 1;
                let suit = (byte & 0b_0011_0000) >> 4;
                match rank {
                    Card::ACE..=Card::KING => match suit {
                        0b00 => Card::Clubs(rank),
                        0b01 => Card::Hearts(rank),
                        0b10 => Card::Spades(rank),
                        0b11 => Card::Diamonds(rank),
                        _ => Card::None // impossible
                    },
                    _ => Card::None
                }
            },
            _ => Card::None // impossible
        }
    }
    pub fn into_byte(&self) -> u8 {
        // for serialization
        match self {
            Card::None => 0b11_00_0000,
            Card::Joker => 0b_10_00_0000,
            Card::Clubs(num) => 0b_00_00_0000 + num,
            Card::Hearts(num) => 0b_00_01_0000 + num,
            Card::Spades(num) => 0b_00_10_0000 + num,
            Card::Diamonds(num) => 0b_00_11_0000 + num
        }
    }

    fn to_none(&mut self) {
        *self = Card::None
    }
    fn to_joker(&mut self) {
        *self = Card::Joker
    }
    fn update(&mut self, card: Card) {
        *self = card;
    }
    /*fn update_rank(&mut self, rank: u8) {
        match self {
            Card::None | Card::Joker => (),
            _ => match rank {
                Card::ACE..=Card::KING => {
                    let mut old_rank: Option<&mut u8> = (*self).mut_num();
                    match old_rank {
                        Some(&mut num) => num = rank,
                        None => ()
                    }
                },
                _ => ()
            }
        }
    }*/

    // getter helpers
    fn suit_name(&self) -> String {
        match self {
            Card::None => "",
            Card::Joker => "Joker",
            Card::Clubs(_) => "Clubs",
            Card::Hearts(_) => "Hearts",
            Card::Spades(_) => "Spades",
            Card::Diamonds(_) => "Diamonds",
        }.to_string()
    }
    /*fn mut_num(&mut self) -> Option<&mut u8> {
        match self {
            Card::Clubs(num)
            | Card::Hearts(num)
            | Card::Spades(num)
            | Card::Diamonds(num) => Some(&mut num),
            Card::Joker
            | Card::None => None,
        }
    }*/
    pub fn num(&self) -> Option<u8> {
        match self {
            Card::Clubs(num)
            | Card::Hearts(num)
            | Card::Spades(num)
            | Card::Diamonds(num) => Some(*num),
            Card::Joker
            | Card::None => None,
        }
    }

    fn suit_sym(&self) -> char {
        match self {
            Card::None
            | Card::Joker => ' ',
            Card::Clubs(_) => '♣',
            Card::Hearts(_) => '♥',
            Card::Spades(_) => '♠',
            Card::Diamonds(_) => '♦'
        }
    }

    fn rank_name(&self) -> String {
        match self.num().unwrap_or(0) {
            1 => "Ace",
            2 => "Two",
            3 => "Three",
            4 => "Four",
            5 => "Five",
            6 => "Six",
            7 => "Seven",
            8 => "Eight",
            9 => "Nine",
            10 => "Ten",
            11 => "Jack",
            12 => "Queen",
            13 => "King",
            _ => "Invalid"
        }.to_string()
    }

    fn rank_sym(&self) -> String {
        format!("{:<2}", match self.num() {
            Some(1) => "A".to_string(),
            Some(11) => "J".to_string(),
            Some(12) => "Q".to_string(),
            Some(13) => "K".to_string(),
            Some(num) => num.to_string(),
            None => match *self {
                Card::Joker => "★".to_string(),
                Card::None => "".to_string(),
                _ => "".to_string()
            }
        })
    }

    // old
    fn _draw_small(&self) -> String {
        format!("[{:<2} {:>2}]", self.rank_sym(), self.suit_sym())
    }

    // display
    fn _name(&self) -> String {
        match self {
            Card::None => String::from("None"),
            Card::Joker => String::from("Joker"),
            Card::Clubs(_) => format!("{} of {}", self.rank_name(), self.suit_name()),
            Card::Hearts(_) => format!("{} of {}", self.rank_name(), self.suit_name()),
            Card::Spades(_) => format!("{} of {}", self.rank_name(), self.suit_name()),
            Card::Diamonds(_) => format!("{} of `{}`", self.rank_name(), self.suit_name())
        }
    }

    pub fn name(&self) -> String {
        match self {
            Card::None => String::from("None"),
            Card::Joker => String::from("Joker"),
            Card::Clubs(_) => format!("{} of Clubs", self.rank_name()),
            Card::Hearts(_) => format!("{} of Hearts", self.rank_name()),
            Card::Spades(_) => format!("{} of Spades", self.rank_name()),
            Card::Diamonds(_) => format!("{} of Diamonds", self.rank_name())
        }
    }

    pub fn draw(&self) -> [String; 5] {
        [
            String::from("┌─────┐"),
            format!("│{:<2}   │", self.rank_sym()),
            format!("│{0:<2} {0:>2}│", self.suit_sym()),
            format!("│   {:>2}│", self.rank_sym()),
            String::from("└─────┘")
        ]
    }
}

mod goofspiel {
    use crate::{Card, input};
    use rand::{seq::SliceRandom, thread_rng};
    use std::cmp::Ordering;



    fn show_cards(cards: &Vec<Card>) {
        let mut output: [String; 5] = Default::default();
        for card in cards {
            for (line_c, line_d) in (*card).draw().iter().zip(&mut output) {
                (*line_d).push_str(&line_c);
            }
        }
        for line in output {
            println!("{}", line);
        }
    }

    fn show_card(card: [String; 5]) {
        let mut output: [String; 5] = Default::default();
        for (line_c, line_d) in card.iter().zip(&mut output) {
            (*line_d).push_str(line_c);
        }
        for line in output {
            println!("{}", line);
        }
    }

    fn new_prize_pool() -> Vec<Card> {
        let mut rng = thread_rng();
        let mut pool: Vec<Card> = (Card::ACE..=Card::KING).map(|x| Card::Clubs(x)).collect();
        pool.shuffle(&mut rng);
        pool
    }

    pub struct Player {
        hand: Vec<Card>,
        pool: Vec<Card>
    }

    impl Player {
        pub fn new() -> Player {
            Player {
                hand: (Card::ACE..=Card::KING).map(|x| Card::Clubs(x)).collect(),
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
                return match card {
                    Card::None => false,
                    _ => true
                }
            }
            false
        }

        pub fn get_choice(&mut self) -> usize {
            loop {
                let choice = input::int("Bid a card by number or enter 0 to pass: ");
                if choice == 0 {
                    return choice;
                } else if self.is_valid_choice(choice) {
                    (*self).hand[choice - 1].to_none();
                    return choice;
                }
            }
        }

        pub fn get_computer_choice(&mut self) -> usize {
            let mut rng = thread_rng();
            let mut valid_choices: Vec<usize> = Vec::new();
            for (i, card) in self.hand.iter().enumerate() {
                match card.num() {
                    Some(_) => valid_choices.push(i),
                    _ => ()
                }
            }
            let choice = valid_choices.choose(&mut rng).unwrap_or(&0);
            (*self).hand[*choice].to_none();
            *choice
        }

        fn accumulate(&self) -> u8 {
            let mut total: u8 = 0;
            for card in &self.pool {
                total += card.num().unwrap_or(0);
            }
            total
        }
    }

    pub fn game() {
        let mut discarded: Vec<Card> = Vec::new();
        // init prize pool
        let mut prizes = new_prize_pool();
        // init new computer
        let mut computer = Player::new_computer();
        // init player and display {pool and hand}
        let mut player = Player::new();
        

        // for card in prize pool reversed:
        //     pop and display prize pool card
        //     ask and compare inputs
        //     display both inputs
        //     award cards (or not)
        let mut show_pool = false;

        loop {
            let prize = prizes.pop().expect("Couldn't get card from prizes pool. :(");
            println!("The prize: ");
            show_card(prize.draw());
            println!("Your hand: ");
            show_cards(&player.hand);
            println!("Your pool so far:");
            if show_pool {
                show_cards(&player.pool);
            }

            let player_choice = player.get_choice();
            let computer_choice = computer.get_computer_choice();

            println!("computer bid: {}", computer_choice);
            println!("you bid: {}", player_choice);
            match player_choice.cmp(&computer_choice) {
                Ordering::Greater => {
                    player.pool.push(prize);
                    println!("Your won the prize");
                    show_pool = true;
                },
                Ordering::Less => computer.pool.push(prize),
                Ordering::Equal => discarded.push(prize)
            }
            if prizes.len() < 1 {
                break;
            }
        }

        let computer_total = computer.accumulate();
        let player_total = player.accumulate();

        println!("Computer total: {}", computer_total);
        println!("Your total: {}", player_total);
        match player_total.cmp(&computer_total) {
            Ordering::Greater => println!("You win!!"),
            Ordering::Less => println!("You lose."),
            Ordering::Equal => println!("Draw!"),
        }

        println!("Computer's pool: ");
        show_cards(&computer.pool);
        println!("Your pool: ");
        show_cards(&player.pool);
        println!("Discarded cards: ");
        show_cards(&discarded);
        // display both hands and pools
        // compare points
        // display win/lose message
    }
}



fn main() {
    loop {
        goofspiel::game();
    }
}
