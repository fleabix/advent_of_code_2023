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
    for data in file_contents.split(',') {
        let mut hash = 0;
        for c in data.chars() {
            hash = (hash + c as u64) * 17 % 256;
        }
        total += hash;
    }
    println!("Part 1: {}", total);

    let mut hashmap: Vec<Vec<(&str, u32)>> = vec![Vec::new(); 256];
    for s in file_contents.split(',') {
        let data: Vec<char> = s.chars().collect();

        let mut hash: usize = 0;
        let mut i = 0;
        loop {
            match data[i] {
                '-' => {
                    println!("hash: {}", hash);
                    hashmap[hash] = hashmap[hash]
                        .iter()
                        .filter(|(v, _)| *v != &s[..i])
                        .map(|(s, l)| (*s, *l))
                        .collect();
                    break;
                }
                '=' => {
                    let f_length = data[i + 1].to_digit(10).unwrap();
                    let mut found = false;
                    for (k, v) in &mut hashmap[hash] {
                        if *k == &s[..i] {
                            found = true;
                            *v = f_length;
                        }
                    }
                    if !found {
                        hashmap[hash].push((&s[..i], f_length));
                    }
                    break;
                }
                _ => {
                    hash = (hash + data[i] as usize) * 17 % 256;
                }
            }
            i = i + 1;
        }

        //println!("{:?}", hashmap);
    }

    let mut total: u64 = 0;
    for box_i in 0..hashmap.len() {
        for lens_i in 0..hashmap[box_i].len() {
            total += ((box_i + 1) * (lens_i + 1) * hashmap[box_i][lens_i].1 as usize) as u64;
        }
    }
    println!("Part 2: {}", total);
}
