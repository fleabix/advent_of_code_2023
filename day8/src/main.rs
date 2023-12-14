use gcd::Gcd;
use nom;
use std::{collections::HashMap, fs::File, io::Read, path::Path};

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
    let (input, directions) =
        nom::character::complete::alpha1::<&str, nom::error::Error<_>>(input).unwrap();
    let (input, _) =
        nom::character::complete::multispace1::<&str, nom::error::Error<_>>(input).unwrap();
    let (_, nodes) =
        nom::multi::separated_list1(nom::character::complete::line_ending, Node::parse)(input)
            .unwrap();

    let directions = directions.chars().collect::<Vec<char>>();
    let nodes_map: HashMap<_, _> = nodes.iter().map(|n| (&n.id, n)).collect();

    let start = nodes
        .iter()
        .filter(|n| n.id.ends_with("A"))
        .collect::<Vec<&Node>>();

    let mut factors: Vec<u64> = Vec::new();
    for mut me in start {
        let mut i = 0;
        while !me.id.ends_with("Z") {
            let next_me = match directions[i % directions.len()] {
                'L' => &me.left,
                'R' => &me.right,
                _ => unreachable!(),
            };
            me = nodes_map[next_me];
            i = i + 1;
        }
        factors.push(i.try_into().unwrap());
    }

    let mut divisors = Vec::new();
    println!("{} {:?}", directions.len(), factors);
    let l: u64 = directions.len().try_into().unwrap();
    for i in 0..factors.len() {
        divisors.push(factors[i] / l);
    }
    println!("{} {:?}", directions.len(), divisors);

    let mut f = factors[0];
    for i in 1..factors.len() {
        let other = factors[i];
        let gcd = f.gcd(other);
        f = f / gcd * other;
    }
    println!("Total: {}", f);
    let mut total = l;
    for n in divisors {
        total = total * n;
    }
    println!("Total: {}", total);
}

#[derive(Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

impl Node {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let (input, id) = nom::character::complete::alpha1(input)?;
        let (input, _) = nom::bytes::complete::tag(" = (")(input)?;
        let (input, left) = nom::character::complete::alpha1(input)?;
        let (input, _) = nom::bytes::complete::tag(", ")(input)?;
        let (input, right) = nom::character::complete::alpha1(input)?;
        let (input, _) = nom::bytes::complete::tag(")")(input)?;
        Ok((
            input,
            Self {
                id: id.to_string(),
                left: left.to_string(),
                right: right.to_string(),
            },
        ))
    }
}
