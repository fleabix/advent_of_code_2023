use std::{fs::File, io::Read, path::Path, cmp, collections::{HashSet, HashMap, VecDeque}};

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

    let mut supports_map: HashMap<u32, HashSet<u32>> = blocks.iter().map(|b| (b.id, HashSet::new())).collect();
    supports_map.insert(0, HashSet::new());

    let mut z_plane = vec![vec![(0u32, 0u32);10];10];
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
                    let z_top = &mut z_plane[x as usize][y as usize];
                    if z_top.1 == max_z {
                        block.sits_on_top_of.insert(z_top.0);
                        supports_map.get_mut(&z_top.0).unwrap().insert(block.id);
                    }

                    z_top.0 = block.id;
                    z_top.1 = max_z + z;
                }
            }
        }
    }

    let blocks_map: HashMap<u32, &Block> = blocks.iter().map(|b| (b.id, b)).collect();
    let mut part1 = 0;
    for (_, supports) in &supports_map {
        let mut can_disintegrate = true;
        for support in supports {
            if blocks_map[support].sits_on_top_of.len() == 1 {
                can_disintegrate = false;
                break;
            }
        }
        if can_disintegrate {
            part1 = part1 + 1;
        }
    }
    println!("Part 1: {}", part1);

    let mut part2 = 0;
    let mut block_queue = VecDeque::new();
    for block_id in blocks_map.keys() {
        let mut fallen_blocks = HashSet::from([block_id]);

        //println!("Checking: {}", block_id);
        block_queue.push_back(block_id);
        while let Some(current_block) = block_queue.pop_front() {
            for blocks_to_check in &supports_map[current_block] {
                let mut stable = false;
                for dependency in &blocks_map[blocks_to_check].sits_on_top_of {
                    if !fallen_blocks.contains(dependency) {
                        stable = true;
                    }
                }
                if stable == false {
                    //println!("Broke: {}", blocks_to_check);
                    fallen_blocks.insert(blocks_to_check);
                    block_queue.push_back(blocks_to_check);
                }
            }
        }
        part2 = part2 + fallen_blocks.len() - 1;
    }
    println!("Part 2: {}", part2);
}

#[derive(Debug)]
struct Block {
    id: u32,
    start: [u32;3],
    end: [u32;3],
    sits_on_top_of: HashSet<u32>,
}