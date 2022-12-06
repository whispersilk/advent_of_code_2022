use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
	let mut file = File::open("input_5.txt")?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	let mut sections = contents.splitn(2, "\n\n");
	let stacks = sections.next().unwrap();
 	let instructions = sections.next().unwrap();
	let mut stacks = get_stacks(stacks);
	move_boxes(&mut stacks, instructions);
	let mut top_boxes = String::with_capacity(stacks.len());
	for stack in stacks.iter() {
		top_boxes.push(*stack.last().unwrap());
	}
	println!("{top_boxes}");
	Ok(())
}

fn get_stacks(stack_section: &str) -> Vec<Vec<char>> {
	let num_stacks = (stack_section.lines().last().unwrap().len() + 1) / 4;
	let mut stacks = Vec::with_capacity(num_stacks);
	for _ in 0..num_stacks {
		stacks.push(Vec::new());
	}
	for line in stack_section.lines().rev().skip(1) {
		for idx in 0..num_stacks {
			let c = line.chars().nth(1 + 4 * idx).unwrap();
			if c != ' ' {
				stacks[idx].push(c);
			}
		}
	};
	stacks
}

fn move_boxes(stacks: &mut Vec<Vec<char>>, instructions: &str) {
	for line in instructions.lines() {
		let mut pieces = line.split(' ').skip(1).step_by(2);
		let iterations = pieces.next().unwrap().parse::<usize>().unwrap();
		let from_stack = pieces.next().unwrap().parse::<usize>().unwrap() - 1;
		let to_stack = pieces.next().unwrap().parse::<usize>().unwrap() - 1;
		for _ in 0..iterations {
			let c = stacks[from_stack].pop().unwrap();
			stacks[to_stack].push(c);
		}
	}
}

#[allow(dead_code)]
fn stringify_stacks(stacks: &Vec<Vec<char>>) -> String {
	let tallest = stacks.iter().map(|s| s.len()).max().unwrap();
	let mut print = String::new();
	for idx in (0..tallest).rev() {
		for stack in stacks.iter() {
			match stack.get(idx) {
				Some(s) => {
					print.push_str(" [");
					print.push(*s);
					print.push(']');
				},
				None => print.push_str("    "),
			}
		}
		print.push('\n');
	}
	print.push('\n');
	for idx in 0..stacks.len() {
		print.push(' ');
		print.push_str(&(idx + 1).to_string());
		print.push_str("  ");
	}
	print
}

/*
Problem:

--- Day 5: Supply Stacks ---

The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.

The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.

The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.

They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:

    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.

Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:

[D]        
[N] [C]    
[Z] [M] [P]
 1   2   3 

In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:

        [Z]
        [N]
    [C] [D]
    [M] [P]
 1   2   3

Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:

        [Z]
        [N]
[M]     [D]
[C]     [P]
 1   2   3

Finally, one crate is moved from stack 1 to stack 2:

        [Z]
        [N]
        [D]
[C] [M] [P]
 1   2   3

The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these together and give the Elves the message CMZ.

After the rearrangement procedure completes, what crate ends up on top of each stack?
*/
