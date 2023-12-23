use std::{
    cmp,
    collections::{HashMap, HashSet, VecDeque},
    env::join_paths,
    fs::File,
    io::Read,
    path::Path,
};

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

    let grid: Vec<Vec<char>> = file_contents.lines().map(|l| l.chars().collect()).collect();
    let mut visited = HashSet::new();

    let mut search_queue = VecDeque::new();
    search_queue.push_front((0, 1, 0));

    let row_max: isize = grid.len() as isize;
    let col_max: isize = grid[0].len() as isize;
    let mut part1 = 0;

    while let Some((row, col, steps)) = search_queue.pop_front() {
        if visited.contains(&(row, col)) {
            visited.remove(&(row, col));
            continue;
        }

        if row as isize == row_max - 1 {
            part1 = cmp::max(part1, steps);
        }

        visited.insert((row, col));
        search_queue.push_front((row, col, steps));
        for new_dir in [Direction::Left, Direction::Right, Direction::Up, Direction::Down] {
            let (next_row, next_col) = new_dir.next(row as isize, col as isize);
            if next_row < 0 || next_col < 0 || next_row >= row_max || next_col >= col_max {
                continue;
            }
            let (next_row, next_col) = (next_row as usize, next_col as usize);
            if visited.contains(&(next_row, next_col)) {
                continue;
            }
            match Direction::from(grid[next_row][next_col]) {
                Direction::None => continue,
                Direction::Any => (),
                //d if d != new_dir => continue,
                _ => (),
            }

            search_queue.push_front((next_row, next_col, steps + 1));
        }
    }
    println!("Part 1: {}", part1);

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Any,
    None,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            '.' => Direction::Any,
            '#' => Direction::None,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    fn next(&self, row: isize, col: isize) -> (isize, isize) {
        match self {
            Direction::Down => (row + 1, col),
            Direction::Up => (row - 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
            _ => unreachable!(),
        }
    }
}
