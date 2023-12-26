use std::collections::{HashMap, HashSet, VecDeque};
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

    let mut nodes = HashMap::new();
    for line in file_contents.lines() {
        let mut split1 = line.split(": ");
        let id = split1.next().unwrap();
        for n in split1.next().unwrap().split(" ") {
            {
                let node = nodes.entry(id).or_insert_with(||HashSet::new());
                node.insert(n.to_owned());
            }

            {
                let node = nodes.entry(n).or_insert_with(||HashSet::new());
                node.insert(id.to_owned());
            }
        }
    }

    let mut visited = HashSet::new();
    let mut search_queue = VecDeque::new();

    search_queue.push_back("mtq");
    while let Some(node) = search_queue.pop_front() {
        visited.insert(node);
        for n in &nodes[node] {
            if !visited.contains(&n.as_str()) {
                search_queue.push_back(n);
            }
        }
    }
    let a = visited.len();
    visited.clear();

    search_queue.push_back("rrz");
    while let Some(node) = search_queue.pop_front() {
        visited.insert(node);
        for n in &nodes[node] {
            if !visited.contains(&n.as_str()) {
                search_queue.push_back(n);
            }
        }
    }
    let b = visited.len();
    println!("Part 2: {}", a * b);
}

