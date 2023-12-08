use std::{path::Path, fs::File, io::Read, cmp};

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

    let total : u32 = s.lines().map(|line| {
        let mut section = line.split(":");
        let _game_num : u32 = section.next().unwrap()[5..].parse::<u32>().unwrap();
        
        let mut blue = 0u32;
        let mut red = 0u32;
        let mut green = 0u32;
        for grab in section.next().unwrap().split(";") {
            for ball in grab.split(",") {
                if ball.ends_with("blue") {
                    let n = ball[..ball.len()-5].trim().parse::<u32>().unwrap();
                    blue = cmp::max(blue, n);
                } else if ball.trim().ends_with("red") {
                    let n = ball[..ball.len()-4].trim().parse::<u32>().unwrap();
                    red = cmp::max(red, n);
                } else if ball.trim().ends_with("green") {
                    let n = ball[..ball.len()-6].trim().parse::<u32>().unwrap();
                    green = cmp::max(green, n);
                }
            }
        }
        blue * red * green
    }).sum();

    println!("Total: {}", total);
}

// fn parse(input: &str) -> nom::IResult<&str, Self> {
//     // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
//     let (next_input, set_elements): (&str, Vec<(u64, &str)>) = nom::multi::separated_list1(
//         nom::bytes::complete::tag(", "),
//         nom::sequence::separated_pair(
//             nom::character::complete::u64,
//             nom::character::complete::char(' '),
//             nom::branch::alt((
//                 nom::bytes::complete::tag("red"),
//                 nom::bytes::complete::tag("green"),
//                 nom::bytes::complete::tag("blue"),
//             )),
//         ),
//     )(input)?;
//     if set_elements.is_empty() || set_elements.len() > 3 {
//         return Err(nom::Err::Error(nom::error::Error::new(
//             input,
//             nom::error::ErrorKind::ManyMN,
//         )));
//     }
//     let input = next_input;
//     let mut red = 0;
//     let mut green = 0;
//     let mut blue = 0;
//     for (number, color) in set_elements {
//         match color {
//             "red" => red = number,
//             "green" => green = number,
//             "blue" => blue = number,
//             _ => unreachable!(),
//         }
//     }
//     Ok((input, Self { red, green, blue }))
// }