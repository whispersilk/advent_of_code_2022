use std::error::Error;
use std::fs::File;
use std::io::Read;

struct Buffer {
	inner: [(usize, char); 14],
}

impl Buffer {
	fn new() -> Buffer {
		Buffer { inner: [(0, '\n'); 14] }
	}

	fn add(&mut self, new_elem: (usize, char)) {
		let least_recent = self.inner.iter().enumerate().min_by(|x, y| x.1.0.cmp(&y.1.0)).unwrap();
		self.inner[least_recent.0] = new_elem;
	}

	fn unique(&self) -> bool {
		self.inner.iter().all(|e| e.1 != '\n' && self.inner.iter().filter(|e2| e.1 == e2.1).count() == 1)
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut file = File::open("input_6.txt")?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	let mut recents: Buffer = Buffer::new();
	let pos = contents.chars().enumerate().take_while(|(idx, c)| {
		recents.add((idx + 1, *c));
		!recents.unique()
	}).count() + 1;
	println!("{pos}");
	Ok(())
}

/*
Problem:

--- Part Two ---

Your device's communication system is correctly detecting packets, but still isn't working. It looks like it also needs to look for messages.

A start-of-message marker is just like a start-of-packet marker, except it consists of 14 distinct characters rather than 4.

Here are the first positions of start-of-message markers for all of the above examples:

    mjqjpqmgbljsphdztnvjfqwrcgsmlb: first marker after character 19
    bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 23
    nppdvjthqldpwncqszvftbrmjlhg: first marker after character 23
    nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 29
    zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 26

How many characters need to be processed before the first start-of-message marker is detected?
*/
