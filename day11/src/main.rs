use std::{cmp, collections::HashSet, fs::File, io::Read, path::Path};

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

    let mut galaxies = HashSet::new();
    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();

    let mut occupied_cols = HashSet::new();
    let mut row = 0;
    let mut col_width = 0;
    for lines in file_contents.lines() {
        let mut col = 0;
        let mut empty_row = true;
        for c in lines.chars() {
            if c == '#' {
                galaxies.insert((row, col));
                occupied_cols.insert(col);
                empty_row = false;
            }
            col += 1;
        }
        if empty_row {
            empty_rows.push(row);
        }
        row += 1;
        col_width = col;
    }

    for i in 0..col_width {
        if !occupied_cols.contains(&i) {
            empty_cols.push(i);
        }
    }

    println!("Empty Rows: {:?}", empty_rows);
    println!("Empty Cols: {:?}", empty_cols);
    println!("Galaxies: {:?}", galaxies);

    let mut total: u64 = 0;
    let galaxies_vec = Vec::from_iter(galaxies);
    for i in 0..galaxies_vec.len() - 1 {
        for j in i + 1..galaxies_vec.len() {
            let (row1, col1) = &galaxies_vec[i];
            let (row2, col2) = &galaxies_vec[j];

            let low_row = cmp::min(row1, row2);
            let high_row = cmp::max(row1, row2);
            let low_col = cmp::min(col1, col2);
            let high_col = cmp::max(col1, col2);

            let mut local = high_row - low_row + high_col - low_col;
            for n in &empty_rows {
                if n < low_row {
                    continue;
                }
                if n < high_row {
                    local += 999999;
                }
                if n > high_row {
                    break;
                }
            }
            for n in &empty_cols {
                if n < low_col {
                    continue;
                }
                if n < high_col {
                    local += 999999;
                }
                if n > high_col {
                    break;
                }
            }

            //println!("Local: ({},{}) ({},{}) {}", row1, col1, row2, col2, local);
            total += local;
        }
    }

    println!("Total: {}", total);
}
