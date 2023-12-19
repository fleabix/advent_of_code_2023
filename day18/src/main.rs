use std::{fs::File, io::Read, path::Path, collections::BinaryHeap};

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

    let mut spans = Vec::new();
    let mut row = 0;
    let mut col = 0;
    for line in file_contents.lines() {
        let mut splits = line.split(' ');
        splits.next();
        splits.next();
        let chars_iter: Vec<char> = splits.next().unwrap().chars().skip(2).collect();
        
        let mut length: i64 = 0;
        for i in 0..5 {
            length = length * 16 + chars_iter[i].to_digit(16).unwrap() as i64;
        }
        match chars_iter[5] {
            '0' => {
                col = col + length;
            },
            '2' => {
                col = col - length;
            },
            '3' => {
                let new_row = row - length;
                spans.push(Span{
                    col,
                    row_start: new_row,
                    row_end: row
                });
                row = new_row;
            },
            '1' => {
                let new_row = row + length;
                spans.push(Span {
                    col, 
                    row_start: row,
                    row_end: new_row
                });
                row = new_row;
            },
            _ => unreachable!(),
        }
    }

    spans.sort_by(|a, b| a.row_start.cmp(&b.row_start));
    
    let mut total = 0;
    let mut min_heap = BinaryHeap::new();

    let mut scanline = spans[0].row_start;
    min_heap.push(spans[0]);
    let mut i = 1;

    while !min_heap.is_empty() {
        min_heap.retain(|s| s.row_end >= scanline);

        while i < spans.len() && spans[i].row_start <= scanline {
            min_heap.push(spans[i]);
            i = i + 1;
        }

        let mut min_heap_consume = min_heap.clone();
        let mut inside = false;
        let mut wall = false;
        let mut start = 0;
        while !min_heap_consume.is_empty() {
            let span = min_heap_consume.pop().unwrap();
            if inside || wall {
                let v = span.col - start - 1;
                //print!("{v} ");
                total += v;
            }

            total = total + 1;
            //print!("1 ");
            if scanline > span.row_start && scanline < span.row_end {
                inside = !inside;
                assert!(wall == false);
            } else if scanline == span.row_start {
                wall = !wall;
            } else if scanline == span.row_end {
                wall = !wall;
                inside = !inside;
            }
            start = span.col;
        }
        //println!();

        scanline = scanline + 1;
    }

    println!("Part 2: {}", total);
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Span {
    col: i64,
    row_start: i64,
    row_end: i64,
}

impl Ord for Span {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.col.cmp(&self.col)
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.col.partial_cmp(&self.col)
    }
}