use std::{path::Path, fs::File, io::Read};
use nom;

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

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in file_contents.lines() {
        grid.push(line.chars().collect());
    }

    let mut start = (0, 0);
    'outer: for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'S' {
                grid[row][col] = '|';
                start = (row, col);
                break 'outer;
            }
        }
    }


    println!("Starting: {:?}", start);
    let mut current = start;
    let mut previous = start;
    let mut steps = 0u32;
    loop {
        let (row, col) = current;
        let next_directions = match &grid[row][col] {
            'F' => [Directions::Down, Directions::Right],
            '-' => [Directions::Left, Directions::Right],
            '7' => [Directions::Left, Directions::Down],
            '|' => [Directions::Up, Directions::Down],
            'J' => [Directions::Left, Directions:: Up],
            'L' => [Directions::Up, Directions::Right],
            _ => unreachable!("fell outta the pipe, send help")
        };

        let mut next = current;
        for step in next_directions {
            next = match step {
                Directions::Down => (row + 1, col),
                Directions::Up => (row - 1, col),
                Directions::Left => (row, col - 1),
                Directions::Right => (row, col + 1),
            };
            if next != previous {
                break;
            }
        }

        steps += 1;
        previous = current;
        current = next;
        if current == start {
            break;
        }
    }
    
    println!("Total: {}", steps / 2);
}

enum Directions {
    Right,
    Down,
    Up,
    Left,
}