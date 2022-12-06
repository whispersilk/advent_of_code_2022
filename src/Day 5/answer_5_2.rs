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
		let start_index = stacks[from_stack].len() - iterations;
		for _ in 0..iterations {
			let c = stacks[from_stack].remove(start_index);
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

--- Part Two ---

As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.

Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.

The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and the ability to pick up and move multiple crates at once.

Again considering the example above, the crates begin in the same configuration:

    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

Moving a single crate from stack 2 to stack 1 behaves the same as before:

[D]        
[N] [C]    
[Z] [M] [P]
 1   2   3 

However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates stay in the same order, resulting in this new configuration:

        [D]
        [N]
    [C] [Z]
    [M] [P]
 1   2   3

Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:

        [D]
        [N]
[C]     [Z]
[M]     [P]
 1   2   3

Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate C that gets moved:

        [D]
        [N]
        [Z]
[M] [C] [P]
 1   2   3

In this example, the CrateMover 9001 has put the crates in a totally different order: MCD.

Before the rearrangement process finishes, update your simulation so that the Elves know where they should stand to be ready to unload the final supplies. After the rearrangement procedure completes, what crate ends up on top of each stack?
*/
