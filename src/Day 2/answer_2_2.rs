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
		let opponent = Move::of(line.chars().next().unwrap());
		acc + Move::score(Outcome::of(me).to_move(opponent.clone()), opponent)
	});
	println!("{total_score}");
	Ok(())
}

#[derive(Clone, PartialEq, Eq)]
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

	fn beats(&self) -> Move {
		match self {
			Move::Rock => Move::Scissors,
			Move::Paper => Move::Rock,
			Move::Scissors => Move::Paper,
		}
	}

	fn loses_to(&self) -> Move {
		match self {
			Move::Rock => Move::Paper,
			Move::Paper => Move::Scissors,
			Move::Scissors => Move::Rock,
		}
	}

	fn ties(&self) -> Move {
		return self.clone()
	}

	fn intrinsic_score(&self) -> u64 {
		match self {
			Move::Rock => 1,
			Move::Paper => 2,
			Move::Scissors => 3,
		}
	}

	fn score(me: Move, opponent: Move) -> u64 {
		me.intrinsic_score() + if me.loses_to() == opponent {
			0
		} else if me.ties() == opponent {
			3
		} else {
			6
		}
	}
}

enum Outcome {
	Win,
	Lose,
	Tie,
}

impl Outcome {
	fn of(s: char) -> Outcome {
		match s {
			'X' => Outcome::Lose,
			'Y' => Outcome::Tie,
			'Z' => Outcome::Win,
			_ => unreachable!(),
		}
	}

	fn to_move(&self, other: Move) -> Move {
		match self {
			Outcome::Win => other.loses_to(),
			Outcome::Lose => other.beats(),
			Outcome::Tie => other.ties(),
		}
	}
}

/*
Problem:

--- Part Two ---

The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

    In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
    In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
    In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.

Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?
*/
