use std::str::FromStr;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};


#[derive(Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scicors,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(input: &str) -> Result<Move, Self::Err> {
        match input {
            "X" => Ok(Move::Rock),
            "Y" => Ok(Move::Paper),
            "Z" => Ok(Move::Scicors),
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scicors),
            _ => Err(())
        }
    }
}


fn lose_hand(this_move: &Move) -> Move {
    match this_move {
        Move::Rock => Move::Scicors,
        Move::Paper => Move::Rock,
        Move::Scicors => Move::Paper
    }
}

fn win_hand(this_move: &Move) -> Move  {
    match this_move {
        Move::Rock => Move::Paper,
        Move::Paper => Move::Scicors,
        Move::Scicors => Move::Rock
    }
}

fn draw_hand(this_move: &Move) -> Move {
    match this_move {
        Move::Rock => Move::Rock,
        Move::Paper => Move::Paper,
        Move::Scicors => Move::Scicors,
    }
}

struct RockPaperScicorsGame {
    opponent_hand: Move, 
    player_hand: Move
}

impl RockPaperScicorsGame {
    fn score_hand(&self) -> i64 {
        let mut score = 0;
        if ((self.opponent_hand == Move::Rock) && (self.player_hand == Move::Paper)) |
            (self.opponent_hand == Move::Paper && self.player_hand == Move::Scicors) |
            (self.opponent_hand == Move::Scicors && self.player_hand == Move::Rock) {
           score += 6; 
        } else if (self.opponent_hand == self.player_hand) {
           score += 3;
        }
        match self.player_hand {
            Move::Rock => score += 1,
            Move::Paper => score += 2,
            Move::Scicors => score +=3
        }
        return score
    }
}

fn parse_hand_part1(line: &str) -> RockPaperScicorsGame {
    println!("The line is {:?}", line);
    let opponent_hand: String = line.chars().nth(0).unwrap().to_string().into();
    let opponent_move: Move = Move::from_str(&opponent_hand).unwrap();

    let player_hand: String = line.chars().nth(2).unwrap().to_string().into(); 
    return RockPaperScicorsGame { 
        opponent_hand: opponent_move,
        player_hand: Move::from_str(&player_hand).unwrap(), 
    }    
}

fn parse_hand_part2(line: &str) -> RockPaperScicorsGame {
    println!("The line is {:?}", line);

    let opponent_hand: String = line.chars().nth(0).unwrap().to_string().into();
    let opponent_move: Move = Move::from_str(&opponent_hand).unwrap();

    let round_win_lose: String = line.chars().nth(2).unwrap().to_string(); 
    let player_move = match round_win_lose.as_str() {
        "X" => lose_hand(&opponent_move),
        "Y" => draw_hand(&opponent_move),
        "Z" => win_hand(&opponent_move),
        _ => panic!("The elves didnt tell me what to do with this :(")
    }; 
    return RockPaperScicorsGame { 
        opponent_hand: opponent_move, 
        player_hand: player_move
    }
}

fn part_one() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut total_score: i64 = 0;
    for line in reader.lines() {
        let game: RockPaperScicorsGame = parse_hand_part1(&line.unwrap());
        total_score += game.score_hand();
    }
    println!("The total score is {:?}", total_score);
}

fn part_two() {
    let file = File::open("./input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut total_score: i64 = 0;
    for line in reader.lines() {
        let game: RockPaperScicorsGame = parse_hand_part2(&line.unwrap());
        total_score += game.score_hand();
    }
    println!("The total score is {:?}", total_score);
}

fn main() -> io::Result<()> {
    part_one();
    part_two();
    Ok(())
}
