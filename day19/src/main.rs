use std::{fs::File, io::Read, path::Path};

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

    let (input, workflows) = nom::multi::separated_list1(
        nom::character::complete::line_ending,
        Workflow::parse,
    )(&file_contents)
    .unwrap();

    let (input, _) =
        nom::character::complete::line_ending::<&str, nom::error::Error<_>>(input).unwrap();
    let (input, _) =
        nom::character::complete::line_ending::<&str, nom::error::Error<_>>(input).unwrap();

    let (_, parts) =
        nom::multi::separated_list1(nom::character::complete::line_ending, Part::parse)(input)
            .unwrap();

    println!("{:#?}", workflows);
    println!("{:#?}", parts);
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let (input, name) = nom::character::complete::alphanumeric1(input)?;
        let (input, _) = nom::bytes::complete::tag("{")(input)?;
        let (input, rules) = nom::multi::separated_list1(
            nom::bytes::complete::tag(","),
            nom::branch::alt((Rule::parse, Rule::parse_target)),
        )(input)?;
        let (input, _) = nom::bytes::complete::tag("}")(input)?;
        Ok((
            input,
            Self {
                name: name.to_string(),
                rules,
            },
        ))
    }
}

#[derive(Debug)]
struct Rule {
    category: char,
    gt: bool,
    value: u64,
    target: String,
}

impl Rule {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let (input, category) = nom::character::complete::one_of("xmas")(input)?;
        let (input, comparator) = nom::character::complete::one_of("<>")(input)?;
        let (input, value) = nom::character::complete::u64(input)?;
        let (input, _) = nom::bytes::complete::tag(":")(input)?;
        let (input, target) = nom::character::complete::alphanumeric1(input)?;

        let gt = comparator == '>';
        Ok((
            input,
            Self {
                category,
                gt,
                value,
                target: target.to_string(),
            },
        ))
    }

    fn parse_target(input: &str) -> nom::IResult<&str, Self> {
        let (input, target) = nom::character::complete::alphanumeric1(input)?;
        Ok((
            input,
            Self {
                category: '_',
                gt: true,
                value: 0,
                target: target.to_string(),
            },
        ))
    }
}

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let (input, _) = nom::bytes::complete::tag("{x=")(input)?;
        let (input, x) = nom::character::complete::u64(input)?;
        let (input, _) = nom::bytes::complete::tag(",m=")(input)?;
        let (input, m) = nom::character::complete::u64(input)?;
        let (input, _) = nom::bytes::complete::tag(",a=")(input)?;
        let (input, a) = nom::character::complete::u64(input)?;
        let (input, _) = nom::bytes::complete::tag(",s=")(input)?;
        let (input, s) = nom::character::complete::u64(input)?;
        let (input, _) = nom::bytes::complete::tag("}")(input)?;

        Ok((input, Self { x, m, a, s }))
    }
}
