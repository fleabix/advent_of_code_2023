use std::{path::Path, fs::File, io::Read, collections::{HashSet, HashMap}, cmp};

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
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (), //print!("{} contains:\n{}", display, s),
    }

    let mut parts = Vec::<PartNumber>::new();
    let mut symbols = HashSet::new();

    let mut y = 0;
    let mut x = 0;
    for line in s.lines() {
        x = 0;
        let mut parsed_part: Option<PartNumber> = None;
        for c in line.chars() {
            match c {
                '0'..='9' => {
                    let n: i32 = c.to_digit(10).unwrap().try_into().unwrap();
                    match parsed_part {
                        None => parsed_part = Some(PartNumber { x: x, y: y, len: 0, value: n }),
                        Some(ref mut part) => {
                            part.len = part.len + 1;
                            part.value = part.value * 10 + n;
                        }
                    }
                } 
                _ => {
                    if parsed_part.is_some() {
                        parts.push(parsed_part.unwrap());
                        parsed_part = None;
                    }
                    if c == '*' {
                        symbols.insert((x, y));
                    }
                }
            }
            x = x + 1;
        }
        if parsed_part.is_some() {
            parts.push(parsed_part.unwrap());
        }
        y = y + 1;
    }

    let mut gears: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    for part in &parts {
        let min_x = cmp::max(part.x - 1, 0);
        let max_x = cmp::min(part.x + part.len + 1, x-1);
        let min_y = cmp::max(part.y - 1, 0);
        let max_y = cmp::min(part.y + 1, y-1);

        // println!("x: {} {}, y: {} {}, v: {}", min_x, max_x, min_y, max_y, part.value);

        let mut found = false;
        for row in min_y..=max_y {
            for col in min_x..=max_x {
                if symbols.contains(&(col, row)) {
                    found = true;
                    if gears.contains_key(&(col, row)) {
                        gears.get_mut(&(col, row)).unwrap().push(part.value)
                    } else {
                        gears.insert((col, row), vec![part.value]);
                    }
                    break;
                }
            }
            if found {
                break;
            }
        }
        // if !found {
        //     println!("Missing: {},{} {}", part.y, part.x, part.value)
        // }
    }
    // println!("Map: {:?}", parts);
    // println!("Symbols: {:?}", symbols);

    let mut total = 0;
    for (_, gear_parts) in gears {
        if gear_parts.len() == 2 {
            total += gear_parts[0] * gear_parts[1];
        }
    }
    println!("Total: {}", total)
}

#[derive(Debug)]
struct PartNumber {
    x: i32,
    y: i32,
    len: i32,
    value: i32
}