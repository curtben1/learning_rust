use std::io;
use rand::Rng;
use crate::PlayerState::{Hollow, Human};
use crate::Transition::{ConsumeHumanity, Die, Kindle, ReverseHollowing};


#[derive(Debug)]
struct Bonfire {
    kindle_level: i32,
}

impl Bonfire {
    fn new() -> Self {
        Self { kindle_level: 0 }
    }

    fn kindle(&mut self) {
        self.kindle_level += 1;
    }
}


#[derive(Debug)]
enum PlayerState {
    Hollow,
    Human,
}

enum Transition<'a> {
    ReverseHollowing,
    Die,
    ConsumeHumanity,
    Kindle { bonfire: &'a mut Bonfire },
}

struct Player {
    state: PlayerState,
    humanity: i32,
    score: i32,
}

impl Player {
    fn new() -> Self {
        Self { state: Hollow, humanity: 10, score: 0 }
    }
    fn action(&mut self, choice: Transition) {
        match (&self.state, self.humanity, self.score, choice) {
            (_, _, _, Die) => {
                self.state = Hollow;
                self.score = 0;
            }
            (_, _, a, ReverseHollowing) if a > 0 => {
                self.state = Human;
                self.score -= 1;
            }
            (_, a, _, ConsumeHumanity) if a > 0 => {
                self.score += 1;
                self.humanity -= 1
            }
            (Human, _, b, Kindle { mut bonfire }) if b > 0 => {
                bonfire.kindle();
                self.score -= 1;
            }
            (_, _, _, _) => {}
        }
    }

    fn print_state(&mut self) {
        println!("score: {}", self.score);
        println!("humanity: {}", self.humanity);
        println!("state: {:?}", self.state);
    }
}


fn main() {
    let mut player = Player::new();
    player.action(ConsumeHumanity);
    player.print_state();
    player.action(ConsumeHumanity);
    player.print_state();
    player.action(ReverseHollowing);
    player.print_state();
    player.action(Die);
    player.print_state();
    player.action(ConsumeHumanity);
    player.action(ConsumeHumanity);
    player.action(ConsumeHumanity);
    player.action(ConsumeHumanity);
    player.action(ConsumeHumanity);
    player.action(ReverseHollowing);
    let mut bonf = Bonfire::new();
    player.print_state();
    player.action(Kindle { bonfire: &mut bonf });
    player.action(Kindle { bonfire: &mut bonf });
    player.print_state();
    println!("{:?}", bonf)
}

fn fizzbuzz() {
    for i in 0..100 {
        let mut print_str = "".to_string();
        if i % 3 == 0 {
            print_str.push_str("fizz")
        }
        if i % 5 == 0 {
            print_str.push_str("buzz")
        }
        if print_str == "".to_string() {
            println!("{i}")
        } else {
            println!("{print_str}", print_str = print_str.as_str());
        }
    }
}

fn rock_paper_scissors() {
    println!("Rock Paper Scissors start?");
    let mut selection = String::new();
    io::stdin().read_line(&mut selection).expect("error");
    let opposition = rand::thread_rng().gen_range(0..3);
    let select_int = match selection.as_str().trim_end() {
        "rock" => 0,
        "paper" => 1,
        "scissors" => 2,
        _ => -1,
    };
    println!("{output}", output = opposition);
    println!("{output}", output = select_int);
    let winner = get_winner(select_int, opposition);
    println!("{output}", output = winner);
    if winner == "invalid entry".to_string() || winner == "draw" {
        rock_paper_scissors();
    }
}

fn get_winner(selection: i32, opposition: i32) -> String {
    match (selection, opposition) {
        (a, b) if (a - 1) % 3 == b => "win".to_string(),
        (a, b) if a == b => "draw".to_string(),
        (-1, _) | (_, -1) => "invalid entry".to_string(),
        _ => "lose".to_string(),
    }
}
