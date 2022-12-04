use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
	let mut file = File::open("input_4.txt")?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	let number_containing = contents.lines().fold(0, |acc, line| {
		let mut pieces = line.splitn(2, ',').map(Range::from_str);
		let first = pieces.next().unwrap();
		let second = pieces.next().unwrap();
		if Range::overlaps(&first, &second) {
			acc + 1
		} else {
			acc
		}
	});
	println!("{number_containing}");
	Ok(())
}

struct Range {
	start: u32,
	end: u32,
}

impl Range {
	// Creates a new Range from a string of the form "<start>-<end>"
	fn from_str(s: &str) -> Range {
		let mut pieces = s.splitn(2, '-');
		let start = pieces.next().unwrap().parse::<u32>().unwrap();
		let end = pieces.next().unwrap().parse::<u32>().unwrap();
		Range {
			start,
			end
		}
	}

	// Returns true if x contains y or y contains x
	#[allow(dead_code)]
	fn contains(x: &Range, y: &Range) -> bool {
		(x.start <= y.start && x.end >= y.end) || (y.start <= x.start && y.end >= x.end)
	}

	// Returns true if x and y overlap
	fn overlaps(x:&Range, y: &Range) -> bool {
		(x.start >= y.start && x.end <= y.end)
		|| (x.end >= y.start && x.end <= y.end)
		|| (y.start >= x.start && y.end <= x.end)
		|| (y.end >= x.start && y.end <= x.end)
	}
}

/*
Problem:

--- Part Two ---

It seems like there is still quite a bit of duplicate work planned. Instead, the Elves would like to know the number of pairs that overlap at all.

In the above example, the first two pairs (2-4,6-8 and 2-3,4-5) don't overlap, while the remaining four pairs (5-7,7-9, 2-8,3-7, 6-6,4-6, and 2-6,4-8) do overlap:

    5-7,7-9 overlaps in a single section, 7.
    2-8,3-7 overlaps all of the sections 3 through 7.
    6-6,4-6 overlaps in a single section, 6.
    2-6,4-8 overlaps in sections 4, 5, and 6.

So, in this example, the number of overlapping assignment pairs is 4.

In how many assignment pairs do the ranges overlap?
*/
