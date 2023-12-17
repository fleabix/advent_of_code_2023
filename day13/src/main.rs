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

    let mut total = 0;
    let boards = file_contents.split("\n\n");

    for board in boards {
        let mut row_sum = Vec::new();
        let mut col_sum = vec![0u32; board.lines().next().unwrap().len()];
        for line in board.lines() {
            row_sum.push(
                line
                    .chars()
                    .map(|c| match c {
                        '#' => 1u32,
                        _ => 0u32,
                    })
                    .fold(0, |acc, x| acc * 2 + x),
            );
    
            let mut i = 0;
            line.chars().map(|c| match c {
                '#' => 1u32,
                _ => 0u32,
            }).for_each(|n| {
                col_sum[i] = col_sum[i] * 2 + n;
                i = i + 1;
            });
        }

        //println!("Board: {:?} {:?}", row_sum, col_sum);

        let mut found = false;
        for i in 0..row_sum.len() - 1 {
            let mut smudge = None;
            for k in 0..=i {
                if i + 1 + k == row_sum.len() {
                    break;
                }
                if row_sum[i-k] != row_sum[i + 1 + k] {
                    match smudge {
                        Some(_) => {
                            smudge = None;
                            break;
                        }
                        None => {
                            let xor = row_sum[i-k] ^ row_sum[i + 1 + k];
                            if xor & (xor - 1) == 0 {
                                smudge = Some(i);
                                println!("Smudge {} {}", i, i-k);
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
            if smudge.is_some() {
                total += 100 * (i + 1);
                println!("Found row {}", i);
                found = true;
                break;
            }
        }
        if found {
            continue;
        }
        for i in 0..col_sum.len() - 1 {
            let mut smudge = None;
            for k in 0..=i {
                if i + 1 + k == col_sum.len() {
                    break;
                }
                if col_sum[i-k] != col_sum[i + 1 + k] {
                    match smudge {
                        Some(_) => {
                            smudge = None;
                            break;
                        }
                        None => {
                            let xor = col_sum[i-k] ^ col_sum[i + 1 + k];
                            if xor & (xor - 1) == 0 {
                                smudge = Some(i);
                                println!("Smudge {} {}", i, i-k);
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
            if smudge.is_some() {
                total += i + 1;
                println!("Found column {}", i);
                found = true;
                break;
            }
        }
        if !found {
            println!("Unfounded: {:#?}", board);
        }
    }
    
    println!("Total: {}", total);
}

