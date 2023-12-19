use std::{fs::File, io::Read, path::Path, collections::{HashSet, BinaryHeap}};

fn main() {
    // Create a path to the desired file
    let path = Path::new("input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }

    let rows: Vec<Vec<u8>> = file_contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    let mut queue = BinaryHeap::new();
    queue.push(
        Block {
            row: 0,
            col: 0,
            dir: Direction::Right,
            dir_n: 0,
            heat: 0,
        },
    );

    let row_max = rows.len() as isize;
    let col_max = rows[0].len() as isize;
    let mut total = 0;
    let mut visited = HashSet::new();
    while !queue.is_empty() {
        let block = queue.pop().unwrap();
        if block.row == row_max - 1 && block.col == col_max - 1 {
            total = block.heat;
            break;
        }

        let v = (block.row, block.col, block.dir, block.dir_n);
        if visited.contains(&v) {
            continue;
        }
        visited.insert(v);

        //println!("Visit: ({},{}) {:?} {} {}", block.row, block.col, block.dir, block.dir_n, block.heat);

        for new_dir in [
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ] {
            if new_dir == block.dir.reverse() {
                continue;
            }
            let (new_row, new_col) = new_dir.next(block.row, block.col);
            if new_row < 0 || new_col < 0 || new_row == row_max || new_col == col_max {
                continue;
            }
            let new_dir_n = if new_dir == block.dir {
                block.dir_n + 1
            } else {
                1
            };
            if new_dir_n > 3 {
                continue;
            }
            let new_heat = block.heat + rows[new_row as usize][new_col as usize] as u64;
            queue.push(
                Block {
                    row: new_row,
                    col: new_col,
                    dir: new_dir,
                    dir_n: new_dir_n,
                    heat: new_heat,
                }
            );
        }
    }

    println!("Part 1: {}", total);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Block {
    row: isize,
    col: isize,
    dir: Direction,
    dir_n: u8,
    heat: u64
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat.cmp(&self.heat)
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.heat.partial_cmp(&self.heat)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn reverse(&self) -> Self {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn next(&self, row: isize, col: isize) -> (isize, isize) {
        match self {
            Direction::Down => (row + 1, col),
            Direction::Up => (row - 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        }
    }
}
