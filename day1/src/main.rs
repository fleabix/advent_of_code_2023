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
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (), //print!("{} contains:\n{}", display, s),
    }

    let total: u32 = s
        .lines()
        .map(|line| {
            let mut first: Option<u32> = None;
            let mut second = 0;
            let mut i = 0;

            while i < line.len() {
                let mut n: Option<u32> = None;
                if let Some(d) = line.chars().nth(i).unwrap().to_digit(10) {
                    n = Some(d);
                } else if line[i..].starts_with("one") {
                    n = Some(1);
                } else if line[i..].starts_with("two") {
                    n = Some(2);
                } else if line[i..].starts_with("three") {
                    n = Some(3);
                } else if line[i..].starts_with("four") {
                    n = Some(4);
                } else if line[i..].starts_with("five") {
                    n = Some(5);
                } else if line[i..].starts_with("six") {
                    n = Some(6);
                } else if line[i..].starts_with("seven") {
                    n = Some(7);
                } else if line[i..].starts_with("eight") {
                    n = Some(8);
                } else if line[i..].starts_with("nine") {
                    n = Some(9);
                }
                i += 1;

                if let Some(d) = n {
                    if first.is_none() {
                        first = Some(d);
                    }
                    second = d;
                }
            }
            first.unwrap() * 10 + second
        })
        .sum();

    println!("Total: {}", total);
}
