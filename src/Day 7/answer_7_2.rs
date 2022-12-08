use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

type Entry = (PathBuf, u64, Type);

#[derive(PartialEq)]
enum Type {
    Dir,
    File,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("input_7.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut entries: Vec<Entry> = vec![("/".into(), 0, Type::Dir)];
    let mut current_entry = 0;
    for line in contents.lines() {
        if line.starts_with("$ cd ..") {
            let mut new_path = entries.get(current_entry).unwrap().0.clone();
            new_path.pop();
            current_entry = add_dir(&mut entries, new_path);
        } else if line.starts_with("$ cd ") {
            let mut new_path = entries.get(current_entry).unwrap().0.clone();
            new_path.push(line.split(" ").nth(2).unwrap());
            current_entry = add_dir(&mut entries, new_path);
        } else if line.starts_with("$ ls") {
            // do nothing
        } else if line.starts_with("dir ") {
            let mut new_path = entries.get(current_entry).unwrap().0.clone();
            new_path.push(line.split(" ").nth(1).unwrap());
            add_dir(&mut entries, new_path);
        } else {
            let mut new_path = entries.get(current_entry).unwrap().0.clone();
            new_path.push(line.split(" ").nth(1).unwrap());
            let size = line.split(" ").next().unwrap().parse::<u64>().unwrap();
            add_file(&mut entries, new_path, size);
        }
    }
    let total_size = entries
        .iter()
        .filter(|e| e.2 == Type::File)
        .fold(0, |acc, e| acc + e.1);
    let need_to_clear = 30_000_000 - (70_000_000 - total_size);
    let smallest_suitable_dir_size = entries
        .iter()
        .filter(|e| e.1 >= need_to_clear)
        .min_by(|e1, e2| e1.1.cmp(&e2.1))
        .unwrap()
        .1;
    println!("{smallest_suitable_dir_size}");
    Ok(())
}

fn add_dir(entries: &mut Vec<Entry>, path: PathBuf) -> usize {
    let exists = entries.iter().position(|e| e.0 == path);
    if exists.is_some() {
        exists.unwrap()
    } else {
        entries.push((path, 0, Type::Dir));
        entries.len() - 1
    }
}

fn add_file(entries: &mut Vec<Entry>, path: PathBuf, size: u64) -> usize {
    let exists = entries.iter().position(|e| e.0 == path);
    if exists.is_some() {
        exists.unwrap()
    } else {
        entries
            .iter_mut()
            .filter(|e| path.starts_with(&e.0))
            .for_each(|e| e.1 += size);
        entries.push((path, size, Type::File));
        entries.len() - 1
    }
}

/*
Problem:

--- Part Two ---

Now, you're ready to choose a directory to delete.

The total disk space available to the filesystem is 70000000. To run the update, you need unused space of at least 30000000. You need to find a directory you can delete that will free up enough space to run the update.

In the example above, the total size of the outermost directory (and thus the total amount of used space) is 48381165; this means that the size of the unused space must currently be 21618835, which isn't quite the 30000000 required by the update. Therefore, the update still requires a directory with total size of at least 8381165 to be deleted before it can run.

To achieve this, you have the following options:

    Delete directory e, which would increase unused space by 584.
    Delete directory a, which would increase unused space by 94853.
    Delete directory d, which would increase unused space by 24933642.
    Delete directory /, which would increase unused space by 48381165.

Directories e and a are both too small; deleting them would not free up enough space. However, directories d and / are both big enough! Between these, choose the smallest: d, increasing unused space by 24933642.

Find the smallest directory that, if deleted, would free up enough space on the filesystem to run the update. What is the total size of that directory?
*/
