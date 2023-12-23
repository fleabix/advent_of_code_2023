use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::Read,
    path::Path, cmp,
};

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

    let grid: Vec<Vec<char>> = file_contents.lines().map(|l| l.chars().collect()).collect();
    let mut visited = HashSet::new();

    let mut search_queue = VecDeque::new();
    search_queue.push_front((0, 1, 0, 0, 1));

    let row_max: isize = grid.len() as isize;
    let col_max: isize = grid[0].len() as isize;

    let mut nodes_map = HashMap::new();
    nodes_map.insert((0, 1), HashMap::new());

    while let Some((row, col, mut steps, mut from_node_row, mut from_node_col)) = search_queue.pop_front() {
        if visited.contains(&(row, col)) {
            visited.remove(&(row, col));
            continue;
        }

        visited.insert((row, col));
        search_queue.push_front((row, col, steps, from_node_row, from_node_col));

        let mut paths = Vec::new();
        let mut eligibles = 0;
        for new_dir in [Direction::Left, Direction::Right, Direction::Up, Direction::Down] {
            let (next_row, next_col) = new_dir.next(row as isize, col as isize);
            if next_row < 0 || next_col < 0 || next_row >= row_max || next_col >= col_max {
                continue;
            }
            let (next_row, next_col) = (next_row as usize, next_col as usize);
            if visited.contains(&(next_row, next_col)) {
                continue;
            }
            match Direction::from(grid[next_row][next_col]) {
                Direction::None => continue,
                d => {
                    eligibles = eligibles + 1;
                    if d == Direction::Any || d == new_dir {
                        paths.push((next_row, next_col));
                    }
                },
            }
        }

        if eligibles > 1 || row as isize == row_max - 1 {
            let coord = (row, col);
            let paths = match nodes_map.get_mut(&coord) {
                Some(p) => p,
                None => {
                    let new_path = HashMap::new();
                    nodes_map.insert(coord.clone(), new_path);
                    nodes_map.get_mut(&coord).unwrap()
                }
            };
            paths.insert((from_node_row, from_node_col), steps);
            nodes_map.get_mut(&(from_node_row, from_node_col)).unwrap().insert(coord, steps);

            steps = 0;
            from_node_row = row;
            from_node_col = col;
        }

        for (new_row, new_col) in paths {
            search_queue.push_front((new_row, new_col, steps + 1, from_node_row, from_node_col));
        }
    }

    let mut part2 = 0;
    search_queue.push_front((0, 1, 0, 0, 1));
    while let Some((row, col, steps, _, _)) = search_queue.pop_front() {
        if visited.contains(&(row, col)) {
            visited.remove(&(row, col));
            continue;
        }

        if row as isize == row_max - 1 {
            part2 = cmp::max(part2, steps);
            continue;
        }
        visited.insert((row, col));
        search_queue.push_front((row, col, steps, 0, 0));

        let mut paths_sort: Vec<(&(usize, usize), &u32)> = nodes_map[&(row, col)].iter().collect();
        paths_sort.sort_by(|(_, a),(_,b)| b.cmp(a));

        for ((dst_row, dst_col), inc_steps) in paths_sort {
            let coord = (*dst_row, *dst_col);
            if !visited.contains(&coord) {
                search_queue.push_front((coord.0, coord.1, steps + inc_steps, 0, 0));
            }
        }
    }
    println!("Part 2: {}", part2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Any,
    None,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            '.' => Direction::Any,
            '#' => Direction::None,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    fn next(&self, row: isize, col: isize) -> (isize, isize) {
        match self {
            Direction::Down => (row + 1, col),
            Direction::Up => (row - 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
            _ => unreachable!(),
        }
    }
}
