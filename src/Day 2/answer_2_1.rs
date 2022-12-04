use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
	let mut file = File::open("input_2.txt")?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	let total_score = contents.lines().fold(0, |acc, line| {
		assert!(line.len() >= 3);
		let me = line.chars().nth(2).unwrap();
		let opponent = line.chars().next().unwrap();
		acc + Move::score(Move::of(me), Move::of(opponent))
	});
	println!("{total_score}");
	Ok(())
}

enum Move {
	Rock,
	Paper,
	Scissors,
}

impl Move {
	fn of(s: char) -> Move {
		match s {
			'A' | 'X' => Move::Rock,
			'B' | 'Y' => Move::Paper,
			'C' | 'Z' => Move::Scissors,
			_ => unreachable!(),
		}
	}

	fn score(me: Move, opponent: Move) -> u64 {
		match (me, opponent) {
			(Self::Rock, Self::Rock) => 4, // 1 for rock, 3 because rock ties rock
			(Self::Rock, Self::Paper) => 1, // 1 for rock, 0 because rock loses to paper
			(Self::Rock, Self::Scissors) => 7, // 1 for rock, 6 because rock beats scissors 
			(Self::Paper, Self::Rock) => 8, // 2 for paper, 6 because paper beats rock
			(Self::Paper, Self::Paper) => 5, // 2 for paper, 3 because paper ties paper
			(Self::Paper, Self::Scissors) => 2, // 2 for paper, 0 because paper loses to scissors
			(Self::Scissors, Self::Rock) => 3, // 3 for scissors, 0 because scissors loses to rock
			(Self::Scissors, Self::Paper) => 9, // 3 for scissors, 6 because scissors beats paper
			(Self::Scissors, Self::Scissors) => 6, // 3 for scissors, 3 because scissors ties scissors
		}
	}
}

/*
Problem:

--- Day 2: Rock Paper Scissors ---

The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z

This strategy guide predicts and recommends the following:

    In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
    In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
    The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.

In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your strategy guide?
*/
