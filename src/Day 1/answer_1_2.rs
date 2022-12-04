use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
	let mut file = File::open("input_1.txt")?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	let elves: (u64, u64, u64) = contents
		.trim()
		.split("\n\n")
		.map(|one_elf|
			one_elf.split('\n')
				.map(|line| u64::from_str_radix(line, 10).unwrap())
				.fold(0, |acc, x| acc + x))
		.fold((0, 0, 0), |acc, x| {
			if x > acc.0 {
				(x, acc.0, acc.1)
			} else if x > acc.1 {
				(acc.0, x, acc.1)
			} else if x > acc.2 {
				(acc.0, acc.1, x)
			} else {
				acc
			}
		});
	println!("{}", elves.0 + elves.1 + elves.2);
	Ok(())
}

/*
Problem:

--- Part Two ---

By the time you calculate the answer to the Elves' question, they've already realized that the Elf carrying the most Calories of food might eventually run out of snacks.

To avoid this unacceptable situation, the Elves would instead like to know the total Calories carried by the top three Elves carrying the most Calories. That way, even if one of those Elves runs out of snacks, they still have two backups.

In the example above, the top three Elves are the fourth Elf (with 24000 Calories), then the third Elf (with 11000 Calories), then the fifth Elf (with 10000 Calories). The sum of the Calories carried by these three elves is 45000.

Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
*/
