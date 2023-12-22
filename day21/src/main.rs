use std::{
    collections::{HashMap, VecDeque},
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

    let mut grid: Vec<Vec<char>> = file_contents.lines().map(|l|l.chars().collect()).collect();
    let row_max = grid.len() as isize;
    let col_max = grid[0].len() as isize;

    let mut start_row = 0;
    let mut start_col = 0;
    let mut found = false;
    for row in 0..row_max as usize {
        for col in 0..col_max as usize {
            if grid[row][col] == 'S' {
                start_row = row as isize;
                start_col = col as isize;
                grid[row][col] = '.';
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }

    let mut visited = HashMap::new();
    let mut work_queue = VecDeque::new();
    work_queue.push_back((start_row, start_col, 0));

    while let Some((row, col, steps)) = work_queue.pop_front() {
        if row < 0 || col < 0 || row >= row_max || col >= col_max || grid[row as usize][col as usize] == '#' {
            continue;
        }
        let cell = (row, col);
        if visited.contains_key(&cell) {
            continue;
        }
        visited.insert(cell, steps);
    
        for (row_step, col_step) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            work_queue.push_back((row + row_step, col + col_step, steps + 1));
        }
    }

    let p1 = visited
        .values()
        .filter(|v| **v <= 64 && **v % 2 == 0)
        .count();

    println!("Part 1: {}", p1);

    //
    // STOLEN from https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
    //
    let even_corners = visited
        .values()
        .filter(|v| **v % 2 == 0 && **v > 65)
        .count();
    let odd_corners = visited
        .values()
        .filter(|v| **v % 2 == 1 && **v > 65)
        .count();

    // This is 202300 but im writing it out here to show the process
    let n = ((26501365 - (row_max / 2)) / row_max) as usize;
    assert_eq!(n, 202300);

    let even = n * n;
    let odd = (n + 1) * (n + 1);

    let p2 = odd * visited.values().filter(|v| **v % 2 == 1).count()
        + even * visited.values().filter(|v| **v % 2 == 0).count()
        - ((n + 1) * odd_corners)
        + (n * even_corners);

    println!("Part 2: {}", p2);
}
