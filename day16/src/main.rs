use std::{fs::File, io::Read, path::Path};

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
                .map(|c| match c {
                    '.' => Tile::Space as u8,
                    '\\' => Tile::Backslash as u8,
                    '/' => Tile::Slash as u8,
                    '|' => Tile::Pipe as u8,
                    '-' => Tile::Dash as u8,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    // let mut total = 0;
    // let row = 0;
    // let col = 0;
    // let direction = Direction::Right;
    // beam(row, col, direction, &mut rows);

    // for row in rows {
    //     for t in row {
    //         if t & 0xF0 != 0 {
    //             total = total + 1;
    //         }
    //     }
    // }

    // println!("Part 1: {}", total);

    let mut grand_total = 0;
    for row in 0..rows.len() {
        let mut subtotal = 0;
        let col = 0;
        let direction = Direction::Right;
        let mut grid = rows.clone();
        beam(row as isize, col, direction, &mut grid);

        for row in grid {
            for t in row {
                if t & 0xF0 != 0 {
                    subtotal = subtotal + 1;
                }
            }
        }
        if subtotal > grand_total {
            grand_total = subtotal;
        }
    }

    for row in 0..rows.len() {
        let mut subtotal = 0;
        let col = rows[0].len() - 1;
        let direction = Direction::Left;
        let mut grid = rows.clone();
        beam(row as isize, col as isize, direction, &mut grid);

        for row in grid {
            for t in row {
                if t & 0xF0 != 0 {
                    subtotal = subtotal + 1;
                }
            }
        }
        if subtotal > grand_total {
            grand_total = subtotal;
        }
    }

    for col in 0..rows[0].len() {
        let mut subtotal = 0;
        let row = 0;
        let direction = Direction::Down;
        let mut grid = rows.clone();
        beam(row, col as isize, direction, &mut grid);

        for row in grid {
            for t in row {
                if t & 0xF0 != 0 {
                    subtotal = subtotal + 1;
                }
            }
        }
        if subtotal > grand_total {
            grand_total = subtotal;
        }
    }

    for col in 0..rows[0].len() {
        let mut subtotal = 0;
        let row = rows.len() - 1;
        let direction = Direction::Down;
        let mut grid = rows.clone();
        beam(row as isize, col as isize, direction, &mut grid);

        for row in grid {
            for t in row {
                if t & 0xF0 != 0 {
                    subtotal = subtotal + 1;
                }
            }
        }
        if subtotal > grand_total {
            grand_total = subtotal;
        }
    }

    println!("Part 2: {}", grand_total);
}

fn beam(mut row: isize, mut col: isize, mut direction: Direction, rows: &mut Vec<Vec<u8>>) {
    loop {
        // bounds check
        if row < 0 || col < 0 || row as usize == rows.len() || col as usize == rows[0].len() {
            return;
        }

        // visited check
        let mut visited = rows[row as usize][col as usize] & 0xF0;
        if direction.contains(visited) {
            return;
        }

        // update visited
        let tile = rows[row as usize][col as usize] & 0xF;
        visited = visited | direction as u8;
        rows[row as usize][col as usize] = visited | tile;

        let tile = Tile::from(tile);
        direction = match tile {
            Tile::Space => direction,
            Tile::Backslash => match direction {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            },
            Tile::Slash => match direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            },
            Tile::Pipe => match direction {
                Direction::Up | Direction::Down => direction,
                Direction::Left | Direction::Right => {
                    let (new_row, new_col) = Direction::Up.next(row, col);
                    beam(new_row, new_col, Direction::Up, rows);
                    Direction::Down
                }
            },
            Tile::Dash => match direction {
                Direction::Up | Direction::Down => {
                    let (new_row, new_col) = Direction::Right.next(row, col);
                    beam(new_row, new_col, Direction::Right, rows);
                    Direction::Left
                }
                Direction::Left | Direction::Right => direction,
            },
        };
        (row, col) = direction.next(row, col);
    }
}

enum Tile {
    Space = 0,
    Backslash = 1,
    Slash = 2,
    Pipe = 3,
    Dash = 4,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            0 => Tile::Space,
            1 => Tile::Backslash,
            2 => Tile::Slash,
            3 => Tile::Pipe,
            4 => Tile::Dash,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up = 0b10000,
    Down = 0b100000,
    Left = 0b1000000,
    Right = 0b10000000,
}

impl Direction {
    fn contains(&self, storage: u8) -> bool {
        let value = *self as u8;
        (storage & value) != 0
    }

    fn next(&self, row: isize, col: isize) -> (isize, isize) {
        match self {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
        }
    }
}
