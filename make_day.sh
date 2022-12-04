if [[ -z "${1+x}" ]]; then
    echo "Must be called with an argument"
    exit 1
fi

DIR="src/Day $1"
mkdir "$DIR"
cat << EOF > "$DIR/answer_$1_1.rs"
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
	let mut file = File::open("input_$1.txt")?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;

	Ok(())
}

/*
Problem:


*/
EOF
cp "$DIR/answer_$1_1.rs" "$DIR/answer_$1_2.rs"
touch "$DIR/input_$1.txt"

$EDITOR "$DIR/answer_$1_1.rs" "$DIR/answer_$1_2.rs" "$DIR/input_$1.txt"
