use std::io;
use std::fmt;

enum Hand {
    NONE,
    ROCK,
    SCISSOR,
    PAPER,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Hand::NONE => write!(f, "None"),
            Hand::ROCK => write!(f, "Rock"),
            Hand::SCISSOR => write!(f, "Scissor"),
            Hand::PAPER => write!(f, "Paper"),
        }
    }
}

trait Player {
    fn get_hand(&self) -> Result<Hand, String>;
}

struct Human {
    name: String,
    life: i32,
}

struct Bot {
    name: String,
    life: i32,
}

impl Player for Human {
    fn get_hand(&self) -> Result<Hand, String> {
        let mut buffer = String::new();
        let res = match io::stdin().read_line(&mut buffer) {
            Ok(_) => Ok(Hand::ROCK),
            Err(_) => Err(String::from("some error")),
        };
        res
    }
}

impl Player for Bot {
    fn get_hand(&self) -> Result<Hand, String> {
        Ok(Hand::PAPER)
    }
}

fn play() {
    let human = Human {
        name: String::from("takashi"),
        life: 3,
    };
    let bot = Bot {
        name: String::from("bot"),
        life: 3,
    };

    println!("start");
    println!("{}, {}, {}, {}", Hand::NONE, Hand::ROCK, Hand::SCISSOR, Hand::PAPER);
    println!("human: {}, {}", human.name, human.life);
    println!("bot: {}, {}", bot.name, bot.life);

    let human_hand = match human.get_hand() {
        Ok(hand) => hand,
        Err(_) => Hand::NONE,
    };
    let bot_hand = match bot.get_hand() {
        Ok(hand) => hand,
        Err(_) => Hand::NONE,
    };

    println!("{}, {}", human_hand, bot_hand);
}

fn main() {
    play();
}
