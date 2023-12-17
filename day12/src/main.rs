use std::{collections::HashMap, fs::File, io::Read, path::Path};

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
    for lines in file_contents.lines() {
        let mut parts = lines.split(" ");
        let springs: Vec<char> = parts.next().unwrap().chars().collect();
        let checksum: Vec<i8> = parts
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        let mut memo = HashMap::new();

        let mut springs_2 = springs.clone();
        let mut checksum_2 = checksum.clone();
        for _ in 0..4 {
            springs_2.push('?');
            springs_2.append(&mut springs.clone());
            checksum_2.append(&mut checksum.clone())
        }

        let valid = count_valid(&mut springs_2, &checksum_2, &mut memo);
        //println!("Valid: {}", valid);
        total += valid;
    }

    println!("Total: {}", total);
}

fn count_valid(
    springs: &mut Vec<char>,
    checksum: &Vec<i8>,
    memo: &mut HashMap<(Vec<char>, Vec<i8>), u64>,
) -> u64 {
    match memo.get(&(springs.clone(), checksum.clone())) {
        Some(total) => {
            return *total;
        }
        None => (),
    }

    let mut s_idx = 0;
    for c_idx in 0..checksum.len() {
        if s_idx == springs.len() {
            return 0;
        }

        while springs[s_idx] == '.' {
            s_idx = s_idx + 1;
            if s_idx == springs.len() {
                return 0;
            }
        }

        // can be anything right now
        if springs[s_idx] == '?' {
            let mut total = 0;

            springs[s_idx] = '#';
            let mut small_springs = springs[s_idx..].to_vec();
            let small_checksum = checksum[c_idx..].to_vec();
            let subtotal = count_valid(&mut small_springs, &small_checksum, memo);
            memo.insert((small_springs, small_checksum), subtotal);
            total += subtotal;

            springs[s_idx] = '.';
            let mut small_springs = springs[s_idx..].to_vec();
            let small_checksum = checksum[c_idx..].to_vec();
            let subtotal = count_valid(&mut small_springs, &small_checksum, memo);
            memo.insert((small_springs, small_checksum), subtotal);
            total += subtotal;

            springs[s_idx] = '?';
            return total;
        }

        // consume # and ? as if it's a #
        for _ in 0..checksum[c_idx] {
            if s_idx == springs.len() {
                return 0;
            }
            if springs[s_idx] == '.' {
                return 0;
            }
            s_idx = s_idx + 1;
        }

        // consume trailing . (or ?) if available
        if s_idx < springs.len() {
            match springs[s_idx] {
                '?' | '.' => {
                    s_idx = s_idx + 1;
                }
                '#' => {
                    return 0;
                }
                _ => unreachable!(),
            }
        }
    }

    while s_idx < springs.len() {
        if springs[s_idx] == '#' {
            return 0;
        }
        s_idx = s_idx + 1;
    }

    return 1;
}
