use std::{collections::HashMap, fs::File, io::Read, path::Path, cmp};

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

    let mut workflows_table = HashMap::new();
    for workflow in workflows {
        workflows_table.insert(workflow.name.clone(), workflow);
    }

    let mut ranges = [(1, 4000); 4];
    let part2 = sum_workflow_accept(&mut ranges, "in", &workflows_table);
    println!("Part 2: {part2}");

    let mut accepted_parts = Vec::new();
    for part in &parts {
        let mut current_flow = "in";
        loop {
            let workflow = &workflows_table[current_flow];
            for rule in &workflow.rules {
                let value = match rule.category {
                    'x' => part.x,
                    'm' => part.m,
                    'a' => part.a,
                    's' => part.s,
                    '_' => {
                        current_flow = &rule.target;
                        break;
                    }
                    _ => unreachable!(),
                };

                if (rule.gt && value > rule.value) || (!rule.gt && value < rule.value) {
                    current_flow = &rule.target;
                    break;
                }
            }
            match current_flow {
                "A" | "R" => {
                    if current_flow == "A" {
                        accepted_parts.push(part);
                    }
                    break;
                }
                _ => (),
            }
        }
    }

    let mut part1 = 0;
    for part in accepted_parts {
        let subtotal = part.x + part.m + part.a + part.s;
        part1 = part1 + subtotal;
    }

    println!("Part 1: {}", part1);
}

fn sum_workflow_accept(
    ranges: &mut [(u64, u64); 4],
    wf: &str,
    workflows: &HashMap<String, Workflow>,
) -> u64 {
    if wf == "A" {
        let mut subtotal = 1u64;
        for (low, high) in ranges {
            print!("({low} {high}) ");
            subtotal = subtotal * (*high - *low + 1) as u64;
        }
        println!();
        return subtotal;
    } else if wf == "R" {
        return 0;
    }
    
    let mut total = 0;
    let workflow = &workflows[wf];
    for rule in &workflow.rules {
        let i = match rule.category {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            '_' => {
                let mut new_ranges = ranges.clone();
                total += sum_workflow_accept(&mut new_ranges, &rule.target, workflows);
                continue;
            },
            _ => unreachable!(),
        };

        let low = cmp::max(rule.value, ranges[i].0);
        let high = cmp::min(rule.value, ranges[i].1);

        // accept block is high block, lower block include
        if rule.gt {
            if low <= ranges[i].1 {
                let mut new_ranges = ranges.clone();
                new_ranges[i].0 = low + 1;
                println!("Pass Rule: {} {} {}", rule.value, ranges[i].0, ranges[i].1);

                total += sum_workflow_accept(&mut new_ranges, &rule.target, workflows);
            } else {
                println!("Pass Filtered All: {} {} {}", rule.value, ranges[i].0, ranges[i].0);
            }
            if high >= ranges[i].0 {
                ranges[i].1 = high;
                println!("Fail Rule: {} {} {}", rule.value, ranges[i].0, ranges[i].1);
                continue;
            } else {
                println!("Fail Filtered All: {} {} {}", rule.value, ranges[i].0, ranges[i].0);

                break;
            }
        } else {
            if high >= ranges[i].0 {
                let mut new_ranges = ranges.clone();
                new_ranges[i].1 = high - 1;
                println!("Pass Rule: {} {} {}", rule.value, ranges[i].0, ranges[i].1);

                total += sum_workflow_accept(&mut new_ranges, &rule.target, workflows);
            } else {
                println!("Pass Filtered All: {} {} {}", rule.value, ranges[i].0, ranges[i].0);

            }
            if low <= ranges[i].1 {
                ranges[i].0 = low;
                println!("Fail Rule: {} {} {}", rule.value, ranges[i].0, ranges[i].1);

                continue;
            } else {
                println!("Fail Filtered All: {} {} {}", rule.value, ranges[i].0, ranges[i].0);
                break;
            }
        }
    }
    total
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
