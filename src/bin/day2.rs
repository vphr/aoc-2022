use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("./src/bin/day2.input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    solve_part1(&contents);
    solve_part2(&contents);
}

#[derive(Debug)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum RPSResult {
    Draw,
    Lose,
    Win,
}
#[derive(Debug)]
struct Opponent {
    hand: Hand,
}

impl Opponent {
    fn new(hand: Hand) -> Self {
        Self { hand }
    }
}
#[derive(Debug)]
struct Player {
    hand: Hand,
    points: u32,
}
impl Player {
    fn new(hand: Hand, points: u32) -> Self {
        Self { hand, points }
    }
    fn new_cheating(opponent: &Opponent, result: RPSResult) -> Self {
        match (&opponent.hand, result) {
            (Hand::Rock, RPSResult::Draw) => Self::new(Hand::Rock, 1),
            (Hand::Rock, RPSResult::Lose) => Self::new(Hand::Scissors, 3),
            (Hand::Rock, RPSResult::Win) => Self::new(Hand::Paper, 2),
            (Hand::Paper, RPSResult::Draw) => Self::new(Hand::Paper, 2),
            (Hand::Paper, RPSResult::Lose) => Self::new(Hand::Rock, 1),
            (Hand::Paper, RPSResult::Win) => Self::new(Hand::Scissors, 3),
            (Hand::Scissors, RPSResult::Draw) => Self::new(Hand::Scissors, 3),
            (Hand::Scissors, RPSResult::Lose) => Self::new(Hand::Paper, 2),
            (Hand::Scissors, RPSResult::Win) => Self::new(Hand::Rock, 1),
        }
    }
    fn compare(&self, opponent: &Opponent) -> u32 {
        let points = &self.points.to_owned();
        match (&self.hand, &opponent.hand) {
            (Hand::Rock, Hand::Rock) => points + 3,
            (Hand::Rock, Hand::Paper) => self.points,
            (Hand::Rock, Hand::Scissors) => points + 6,
            (Hand::Paper, Hand::Rock) => points + 6,
            (Hand::Paper, Hand::Paper) => points + 3,
            (Hand::Paper, Hand::Scissors) => self.points,
            (Hand::Scissors, Hand::Rock) => self.points,
            (Hand::Scissors, Hand::Paper) => self.points,
            (Hand::Scissors, Hand::Scissors) => self.points,
        }
    }
}

fn solve_part1(input: &str) {
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|l| {
            let input = l.split_whitespace().collect::<Vec<&str>>();
            input
        })
        .collect();

    let res = lines
        .into_iter()
        .map(|entry| {
            let first = entry[0];
            let opponent = match first {
                "A" => Opponent::new(Hand::Rock),
                "B" => Opponent::new(Hand::Paper),
                "C" => Opponent::new(Hand::Scissors),
                _ => panic!("should not happen lol"),
            };
            let second = entry[1];
            let player = match second {
                "X" => Player::new(Hand::Rock, 1),
                "Y" => Player::new(Hand::Paper, 2),
                "Z" => Player::new(Hand::Scissors, 3),
                _ => panic!("should not happen lol"),
            };
            player.compare(&opponent)
        })
        .into_iter()
        .sum::<u32>();

    println!("{:?}", res)
}

fn solve_part2(input: &str) {
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|l| {
            let input = l.split_whitespace().collect::<Vec<&str>>();
            input
        })
        .collect();

    let result = lines
        .into_iter()
        .map(|entry| {
            let first = entry[0];
            let opponent = match first {
                "A" => Opponent::new(Hand::Rock),
                "B" => Opponent::new(Hand::Paper),
                "C" => Opponent::new(Hand::Scissors),
                _ => panic!("IMPOSSIBLE"),
            };
            let second = entry[1];
            let player = match second {
                "X" => RPSResult::Lose,
                "Y" => RPSResult::Draw,
                "Z" => RPSResult::Win,
                _ => panic!("IMPOSSIBLE"),
            };

            let hand = Player::new_cheating(&opponent, player);

            hand.compare(&opponent)
        })
        .into_iter()
        .sum::<u32>();

    println!("{:?}", result)
}
