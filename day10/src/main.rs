use std::{path::Path, fs::File, io::Read};

fn main() {
    // Create a path to the desired file
    let path = Path::new("clean_input.txt");
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

    let mut inside = 0;
    for row in &grid {
        let mut dir: Option<Directions> = None;
        let mut walls = 0;
        for col in row {
            match col {
                '-' => (), // all good, carry on
                '.' => {
                    if walls % 2 != 0 {
                        inside += 1;
                    }
                },
                '|' => {
                    match dir {
                        None => walls += 1,
                        _ => panic!("pipes bad {:?}", dir)
                    }
                },
                'F' => {
                    match dir {
                        None => dir = Some(Directions::Up),
                        _ => panic!("pipes bad {:?}", dir)
                    }
                },
                'L' => {
                    match dir {
                        None => dir = Some(Directions::Down),
                        _ => panic!("pipes bad {:?}", dir)
                    }
                }
                'J' => {
                    match dir {
                        Some(Directions::Up) => walls += 1,
                        Some(Directions::Down) => walls += 2,
                        _ => panic!("pipes bad {:?}", dir)
                    }
                    dir = None;
                },
                '7' => {
                    match dir {
                        Some(Directions::Down) => walls += 1,
                        Some(Directions::Up) => walls += 2,
                        _ => panic!("pipes bad {:?}", dir)
                    }
                    dir = None;
                },
                _ => panic!("pipes bad"),
            }
        }
    }

    println!("Number inside: {}", inside);
}

#[derive(PartialEq, Debug)]
enum Directions {
    Down,
    Up,
}