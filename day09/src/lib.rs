pub fn solve(input: &str) -> (u64, u64) {
    let (storage, files) = parse(input);

    (part1(&storage), part2(&storage, &files))
}

fn part1(storage: &[Block]) -> u64 {
    let mut storage = storage.to_vec();
    let mut index = 0;

    // make sure index is one before the last value
    while index < storage.len() - 1 {
        if storage[index] != Block::Empty {
            index += 1;
            continue;
        }

        loop {
            if *storage.last().unwrap() == Block::Empty {
                storage.pop();
            } else {
                if index < storage.len() {
                    storage[index] = storage.pop().expect("IDK");
                }
                break;
            }
        }

        index += 1;
    }

    checksum(&storage)
}

fn part2(storage: &[Block], files: &[File]) -> u64 {
    let mut storage = storage.to_vec();

    'file: for file in files.iter().rev() {
        let mut empty_size = 0;
        let mut new_index = 0;
        let mut has_space = false;

        for (index, block) in storage.iter().enumerate() {
            if index > file.index {
                continue 'file; // no empty valid
            }
            match block {
                Block::Empty => {
                    empty_size += 1;

                    if empty_size == file.size {
                        has_space = true;
                        break;
                    }
                }
                Block::ID(_) => {
                    empty_size = 0;
                    new_index = index + 1;
                }
            }
        }
        if has_space {
            move_file(&mut storage, file, new_index);
        }
    }

    checksum(&storage)
}

fn parse(input: &str) -> (Vec<Block>, Vec<File>) {
    let mut storage = vec![];
    let mut files = vec![];
    let mut id = 0;
    let mut cur_index = 0;

    // length, then free space
    for (index, ch) in input.chars().enumerate() {
        assert!(ch.is_numeric(), "Encountered non-number");
        let ch = ch.to_digit(10).unwrap() as usize;

        if index % 2 == 0 {
            storage.append(&mut vec![Block::ID(id); ch]);
            files.push(File::new(ch, id, cur_index));
            id += 1;
        } else {
            storage.append(&mut vec![Block::Empty; ch]);
        }
        cur_index += ch;
    }

    (storage, files)
}

fn checksum(storage: &[Block]) -> u64 {
    storage
        .iter()
        .enumerate()
        .map(|(index, block)| match block {
            Block::Empty => 0,
            Block::ID(id) => (index as u64) * (*id as u64),
        })
        .sum()
}

fn move_file(storage: &mut [Block], file: &File, index: usize) {
    // copy file to index
    for block in storage.iter_mut().skip(index).take(file.size) {
        *block = Block::ID(file.id);
    }

    // remove file
    for block in storage.iter_mut().skip(file.index).take(file.size) {
        *block = Block::Empty;
    }
}

// for debug
fn _print_storage(storage: &Vec<Block>) {
    println!("Storage: ");
    for block in storage {
        print!(
            "{}",
            match block {
                Block::Empty => ".".to_string(),
                Block::ID(id) => id.to_string(),
            }
        )
    }
    println!();
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Block {
    Empty,
    ID(usize),
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct File {
    size: usize,
    id: usize,
    index: usize,
}

impl File {
    fn new(size: usize, id: usize, index: usize) -> Self {
        File { size, id, index }
    }
}

// 1928, 2858
// 6291146824486, 6307279963620
