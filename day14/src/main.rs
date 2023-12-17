use std::{fs::File, io::Read, path::Path, collections::HashMap};

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

    let mut rows: Vec<Vec<char>> = file_contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // Load North
    let mut total = 0;
    for col in 0..rows[0].len() {
        let mut column_weight = 0;
        let mut space = None;
        for row in 0..rows.len() {
            match &rows[row][col] {
                '#' => space = None,
                '.' => {
                    if space.is_none() {
                        space = Some(row);
                    }
                }
                'O' => {
                    let rock_weight =
                    match space {
                        Some(n) => {
                            space = Some(n + 1);
                            rows.len() - n
                        },
                        None => {
                            rows.len() - row
                        }
                    };
                    column_weight += rock_weight;
                }
                _ => unreachable!(),
            } 
        }
        total += column_weight;
    }
    println!("Part 1: {}", total);

    let mut memo = HashMap::<Vec<Vec<char>>, u64>::new();

    let cycles = 1000;
    let mut cycle_start = 0;
    let mut cycle_period = 0;
    for c in 0.. cycles {
        // North
        for col in 0..rows[0].len() {
            let mut space = None;
            for row in 0..rows.len() {
                match &rows[row][col] {
                    '#' => space = None,
                    '.' => {
                        if space.is_none() {
                            space = Some(row);
                        }
                    }
                    'O' => {
                        match space {
                            Some(n) => {
                                space = Some(n + 1);
                                rows[n][col] = 'O';
                                rows[row][col] = '.';
                            }
                            None => (),
                        };
                    }
                    _ => unreachable!(),
                } 
            }
        }

        // West
        for row in 0..rows.len() {
            let mut space = None;
            for col in 0..rows[0].len() {
                match &rows[row][col] {
                    '#' => space = None,
                    '.' => {
                        if space.is_none() {
                            space = Some(col);
                        }
                    }
                    'O' => {
                        match space {
                            Some(n) => {
                                space = Some(n + 1);
                                rows[row][n] = 'O';
                                rows[row][col] = '.';
                            }
                            None => (),
                        };
                    }
                    _ => unreachable!(),
                } 
            }
        }

        // South
        for col in 0..rows[0].len() {
            let mut space = None;
            for row in (0..rows.len()).rev() {
                match &rows[row][col] {
                    '#' => space = None,
                    '.' => {
                        if space.is_none() {
                            space = Some(row);
                        }
                    }
                    'O' => {
                        match space {
                            Some(n) => {
                                space = Some(n - 1);
                                rows[n][col] = 'O';
                                rows[row][col] = '.';
                            }
                            None => (),
                        };
                    }
                    _ => unreachable!(),
                } 
            }
        }

        // East
        for row in 0..rows.len() {
            let mut space = None;
            for col in (0..rows[0].len()).rev() {
                match &rows[row][col] {
                    '#' => space = None,
                    '.' => {
                        if space.is_none() {
                            space = Some(col);
                        }
                    }
                    'O' => {
                        match space {
                            Some(n) => {
                                space = Some(n - 1);
                                rows[row][n] = 'O';
                                rows[row][col] = '.';
                            }
                            None => (),
                        };
                    }
                    _ => unreachable!(),
                } 
            }
        }

        if memo.contains_key(&rows) {
            cycle_start = memo[&rows];
            cycle_period = c - memo[&rows];
            break;
        }
        memo.insert(rows.clone(), c);

        // Load North
        let mut total = 0;
        for col in 0..rows[0].len() {
            let mut column_weight = 0;
            for row in 0..rows.len() {
                match &rows[row][col] {
                    'O' => {
                        let rock_weight = rows.len() - row;
                        column_weight += rock_weight;
                    }
                    _ => (),
                } 
            }
            total += column_weight;
        }
        println!("Total: {} {}", c, total);

        // for row in 0..rows.len() {
        //     for col in 0..rows[0].len() {
        //         print!("{}", rows[row][col]);
        //     }
        //     println!("");
        // }
        // println!("");
    }

    let launder = (1000000000 - cycle_start - 1) % cycle_period;

    for _ in 0..launder {
        // North
        for col in 0..rows[0].len() {
            let mut space = None;
            for row in 0..rows.len() {
                match &rows[row][col] {
                    '#' => space = None,
                    '.' => {
                        if space.is_none() {
                            space = Some(row);
                        }
                    }
                    'O' => {
                        match space {
                            Some(n) => {
                                space = Some(n + 1);
                                rows[n][col] = 'O';
                                rows[row][col] = '.';
                            }
                            None => (),
                        };
                    }
                    _ => unreachable!(),
                } 
            }
        }

        // West
        for row in 0..rows.len() {
            let mut space = None;
            for col in 0..rows[0].len() {
                match &rows[row][col] {
                    '#' => space = None,
                    '.' => {
                        if space.is_none() {
                            space = Some(col);
                        }
                    }
                    'O' => {
                        match space {
                            Some(n) => {
                                space = Some(n + 1);
                                rows[row][n] = 'O';
                                rows[row][col] = '.';
                            }
                            None => (),
                        };
                    }
                    _ => unreachable!(),
                } 
            }
        }

        // South
        for col in 0..rows[0].len() {
            let mut space = None;
            for row in (0..rows.len()).rev() {
                match &rows[row][col] {
                    '#' => space = None,
                    '.' => {
                        if space.is_none() {
                            space = Some(row);
                        }
                    }
                    'O' => {
                        match space {
                            Some(n) => {
                                space = Some(n - 1);
                                rows[n][col] = 'O';
                                rows[row][col] = '.';
                            }
                            None => (),
                        };
                    }
                    _ => unreachable!(),
                } 
            }
        }

        // East
        for row in 0..rows.len() {
            let mut space = None;
            for col in (0..rows[0].len()).rev() {
                match &rows[row][col] {
                    '#' => space = None,
                    '.' => {
                        if space.is_none() {
                            space = Some(col);
                        }
                    }
                    'O' => {
                        match space {
                            Some(n) => {
                                space = Some(n - 1);
                                rows[row][n] = 'O';
                                rows[row][col] = '.';
                            }
                            None => (),
                        };
                    }
                    _ => unreachable!(),
                } 
            }
        }
    }

    // Load North
    let mut total = 0;
    for col in 0..rows[0].len() {
        let mut column_weight = 0;
        for row in 0..rows.len() {
            match &rows[row][col] {
                'O' => {
                    let rock_weight = rows.len() - row;
                    column_weight += rock_weight;
                }
                _ => (),
            } 
        }
        total += column_weight;
    }

    println!("Total: {}", total);
}
