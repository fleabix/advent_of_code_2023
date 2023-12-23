use std::{fs::File, io::Read, path::Path, cmp, collections::HashSet};

fn main() {
    // Create a path to the desired file
    let path = Path::new("test.txt");
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

    let mut i = 0;
    let mut blocks: Vec<Block> = file_contents
        .lines()
        .map(|line| {
            let mut coords = line.split("~")
                .map(|coord| {
                    coord.split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>()
                });
            i = i + 1;

            Block {
                id: i,
                start: coords.next().unwrap().try_into().unwrap(),
                end: coords.next().unwrap().try_into().unwrap(),
                sits_on_top_of: HashSet::new(),
            }
        })
        .collect();

    blocks.sort_by(|a, b| {
        a.start[2].cmp(&b.start[2])
    });

    let mut z_plane = vec![vec![(0u32, 0u32);3];3];
    for block in &mut blocks {
        let mut max_z = 0;
        for x in block.start[0]..=block.end[0] {
            for y in block.start[1]..=block.end[1] {
                max_z = cmp::max(max_z, z_plane[x as usize][y as usize].1);
            }
        }

        assert!(max_z < block.start[2]);

        for x in block.start[0]..=block.end[0] {
            for y in block.start[1]..=block.end[1] {
                for z in 1..=(block.end[2] - block.start[2] + 1) {
                    if z_plane[x as usize][y as usize].1 == max_z {
                        block.sits_on_top_of.insert(z_plane[x as usize][y as usize].0);
                    }

                    z_plane[x as usize][y as usize].0 = block.id;
                    z_plane[x as usize][y as usize].1 = max_z + z;
                }
            }
        }

        println!("{:?}", z_plane);
    }

    println!("{:#?}", blocks);
}

#[derive(Debug)]
struct Block {
    id: u32,
    start: [u32;3],
    end: [u32;3],
    sits_on_top_of: HashSet<u32>,
}