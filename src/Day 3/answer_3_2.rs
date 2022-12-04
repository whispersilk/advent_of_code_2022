use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
	let mut file = File::open("input_3.txt")?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	let mut inventory_sets = contents.lines()
		.map(|line| line.chars().fold(0, |acc, x| acc | priority(x)))
		.peekable();
	let mut badge_sum = 0;
	while inventory_sets.peek().is_some() {
		let first = inventory_sets.next().unwrap();
		let second = inventory_sets.next().unwrap();
		let third = inventory_sets.next().unwrap();
		let overlap = first & second & third;
		badge_sum += overlap_priority(overlap);
	}
	println!("{badge_sum}");
	Ok(())
}

// Returns a u64 with the Xth bit set, where X is the char's priority
fn priority(c: char) -> u64 {
	if c >= 'A' && c <= 'Z' {
		1 << (26 + (c as u8 - 'A' as u8))
	} else if c >= 'a' && c <= 'z' {
		 1 << (c as u8 - 'a' as u8)
	} else {
		0
	}
}

fn overlap_priority(overlap: u64) -> u64 {
	assert!(overlap & overlap - 1 == 0);
	let mut n = overlap;
	let mut priority = 1;
	while n > 1 {
		priority += 1;
		n = n >> 1;
	}
	priority
}

/*
Problem:

--- Part Two ---

As you finish identifying the misplaced items, the Elves come to you with another issue.

For safety, the Elves are divided into groups of three. Every Elf carries a badge that identifies their group. For efficiency, within each group of three Elves, the badge is the only item type carried by all three Elves. That is, if a group's badge is item type B, then all three Elves will have item type B somewhere in their rucksack, and at most two of the Elves will be carrying any other item type.

The problem is that someone forgot to put this year's updated authenticity sticker on the badges. All of the badges need to be pulled out of the rucksacks so the new authenticity stickers can be attached.

Additionally, nobody wrote down which item type corresponds to each group's badges. The only way to tell which item type is the right one is by finding the one item type that is common between all three Elves in each group.

Every set of three lines in your list corresponds to a single group, but each group can have a different badge item type. So, in the above example, the first group's rucksacks are the first three lines:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg

And the second group's rucksacks are the next three lines:

wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

In the first group, the only item type that appears in all three rucksacks is lowercase r; this must be their badges. In the second group, their badge item type must be Z.

Priorities for these items must still be found to organize the sticker attachment efforts: here, they are 18 (r) for the first group and 52 (Z) for the second group. The sum of these is 70.

Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types?
*/