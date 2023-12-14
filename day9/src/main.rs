use nom;
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
    let input = file_contents.as_str();
    let (_, readings) = nom::multi::separated_list1(
        nom::character::complete::line_ending::<&str, nom::error::Error<_>>,
        nom::multi::separated_list1(
            nom::character::complete::space1,
            nom::character::complete::i64,
        ),
    )(input)
    .unwrap();

    let mut total = 0;
    for mut line in readings {
        let mut pyramid = Vec::new();
        pyramid.push(line.clone());
        loop {
            let mut all_zero: bool = true;
            let mut next_line = Vec::new();
            for i in 0..line.len() - 1 {
                let v = line[i + 1] - line[i];
                next_line.push(v);
                if v != 0 {
                    all_zero = false;
                }
            }
            pyramid.push(next_line.clone());
            if all_zero {
                break;
            }
            line = next_line;
        }

        for i in (1..pyramid.len()).rev() {
            let src = pyramid[i][0];
            let dst = &mut pyramid[i - 1];
            dst.insert(0, dst[0] - src);
        }

        //println!("Pyramid: {:?}", pyramid);
        total += pyramid.first().unwrap().first().unwrap();
    }

    println!("Total: {}", total);
}
