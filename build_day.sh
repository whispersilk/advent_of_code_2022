if [[ -z "${1+x}" ]]; then
    echo "Must be called with an argument"
    exit 1
fi

SRC_DIR="src/Day $1"
BUILD_DIR="build/Day $1"
mkdir -p "$BUILD_DIR"
cp "$SRC_DIR/input_$1.txt" "$BUILD_DIR/input_$1.txt"

for file in $(ls "$SRC_DIR" | grep ".rs"); do
    rustc "$SRC_DIR/$file" -o "$BUILD_DIR/$(echo $file | sed 's/.rs//g')"
done
cd "$BUILD_DIR"
echo Day $1 part 1: $("./answer_$1_1")
echo Day $1 part 2: $("./answer_$1_2")
