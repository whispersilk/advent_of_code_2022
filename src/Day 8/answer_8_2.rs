use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("input_8.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let map: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();
    let max_view_score = (0..map.len())
        .map(|h| (0..map[h].len()).map(|w| view_score(&map, h, w)).max())
        .max()
        .unwrap()
        .unwrap();
    println!("{max_view_score}");
    Ok(())
}

fn view_score(map: &Vec<Vec<char>>, h: usize, w: usize) -> usize {
    let (height, width, tree) = (map.len(), map[0].len(), map[h][w]);
    let lview = (0..w).rev().take_while(|w| map[h][*w] < tree).count();
    let rview = ((w + 1)..width).take_while(|w| map[h][*w] < tree).count();
    let tview = (0..h).rev().take_while(|h| map[*h][w] < tree).count();
    let bview = ((h + 1)..height).take_while(|h| map[*h][w] < tree).count();
    (lview + if lview == w { 0 } else { 1 })
        * (rview + if rview == width - 1 - w { 0 } else { 1 })
        * (tview + if tview == h { 0 } else { 1 })
        * (bview + if bview == height - 1 - h { 0 } else { 1 })
}

/*
Problem:

--- Part Two ---

Content with the amount of tree cover available, the Elves just need to know the best spot to build their tree house: they would like to be able to see a lot of trees.

To measure the viewing distance from a given tree, look up, down, left, and right from that tree; stop if you reach an edge or at the first tree that is the same height or taller than the tree under consideration. (If a tree is right on the edge, at least one of its viewing distances will be zero.)

The Elves don't care about distant trees taller than those found by the rules above; the proposed tree house has large eaves to keep it dry, so they wouldn't be able to see higher than the tree house anyway.

In the example above, consider the middle 5 in the second row:

30373
25512
65332
33549
35390

    Looking up, its view is not blocked; it can see 1 tree (of height 3).
    Looking left, its view is blocked immediately; it can see only 1 tree (of height 5, right next to it).
    Looking right, its view is not blocked; it can see 2 trees.
    Looking down, its view is blocked eventually; it can see 2 trees (one of height 3, then the tree of height 5 that blocks its view).

A tree's scenic score is found by multiplying together its viewing distance in each of the four directions. For this tree, this is 4 (found by multiplying 1 * 1 * 2 * 2).

However, you can do even better: consider the tree of height 5 in the middle of the fourth row:

30373
25512
65332
33549
35390

    Looking up, its view is blocked at 2 trees (by another tree with a height of 5).
    Looking left, its view is not blocked; it can see 2 trees.
    Looking down, its view is also not blocked; it can see 1 tree.
    Looking right, its view is blocked at 2 trees (by a massive tree of height 9).

This tree's scenic score is 8 (2 * 2 * 1 * 2); this is the ideal spot for the tree house.

Consider each tree on your map. What is the highest scenic score possible for any tree?
*/
