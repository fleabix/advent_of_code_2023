use std::{path::Path, fs::File, io::Read, cmp};
use nom::{self, Finish};
use itertools::Itertools;

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

    let (input, seeds) = Seeds::parse(&file_contents).expect("woof");
    let (input, _) = nom::character::complete::multispace1::<&str, nom::error::Error<_>>(input).finish().unwrap();

    let (_, maps) = nom::multi::separated_list1(nom::character::complete::multispace1, Mapper::parse)(input).unwrap();

    // let mut minimum = u64::MAX;
    // for (start, len) in seeds.seeds.into_iter().tuples() {
    //     for seed in start..start+len {
    //         let mut out = seed;
    //         for mapper in &maps {
    //             out = mapper.map(out);
    //         }
    //         minimum = cmp::min(out, minimum);
    //     }
    // }

    let mut ranges = Vec::new();
    let mut i = 0;
    while i < seeds.seeds.len() {
        ranges.push(Range { start: seeds.seeds[i], len: seeds.seeds[i+1] });
        i += 2;
    }

    for mapper in maps {
        let mut new_ranges = Vec::new();
        for range in ranges {
            new_ranges.append(&mut mapper.map_range(range));
        }
        ranges = new_ranges;
    }

    //println!("Seeds: {:?}", seeds);
    //println!("Mappers: {:?}", maps);
    println!("Minimum: {:?}", ranges);
    println!("Minimum: {}", ranges.iter().map(|a|a.start).min().unwrap());

}

#[derive(Debug)]
struct Range {
    start: u64,
    len: u64
}

#[derive(Debug)]
struct Seeds {
    seeds : Vec<u64>
}

impl Seeds {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let (input, _) = nom::bytes::complete::tag("seeds:")(input)?;
        let (input, _) = nom::character::complete::space1(input)?;
        let (input, seeds) = nom::multi::separated_list1(
            nom::character::complete::space1,
            nom::character::complete::u64,
        )(input)?;
        Ok((input, Self { seeds }))
    }
}

#[derive(Debug)]
struct Mapper {
    _name : String,
    conversions : Vec<(u64, u64, u64)>,
}

impl Mapper {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let (input, source) = nom::character::complete::alpha1(input)?;
        let (input, _) = nom::character::complete::anychar(input)?;
        let (input, to) = nom::character::complete::alpha1(input)?;
        let (input, _) = nom::character::complete::anychar(input)?;
        let (input, destination) = nom::character::complete::alpha1(input)?;
        let (input, _) = nom::bytes::complete::tag(" map:")(input)?;
        let (input, _) = nom::character::complete::line_ending(input)?;
        let (input, mut conversions) =
            nom::multi::separated_list1(
                nom::character::complete::line_ending,
                nom::combinator::map(
                    nom::sequence::tuple((
                        nom::sequence::terminated(nom::character::complete::u64, nom::character::complete::space1),
                        nom::sequence::terminated(nom::character::complete::u64, nom::character::complete::space1),
                        nom::character::complete::u64
                    )),
                |(a, b, c)| (b, a, c)
                )
            )(input)?;
        conversions.sort_by(|a, b| a.0.cmp(&b.0));
        Ok((input, Self { _name: format!("{} {} {}", source, to, destination) , conversions }))
    }

    fn map(&self, from : u64) -> u64 {
        for (src, dst, len) in &self.conversions {
            if from < *src {
                return from;
            }
            if from < src + len {
                return from - src + dst;
            }
        }
        return from;
    }

    fn map_range(&self, from : Range) -> Vec<Range> {
        let mut start = from.start;
        let end = from.start + from.len;        
        let mut new_ranges = Vec::new();

        for (src, dst, len) in &self.conversions {
            let marker = *src;
            if start < marker {
                if end < marker {
                    new_ranges.push(Range{ start: start, len: end - start });
                    return new_ranges;
                } else {
                    new_ranges.push(Range{ start: start, len: marker - start });
                    start = marker;
                }
            }

            let marker = src + len;
            if start < marker {
                if end < marker {
                    new_ranges.push(Range{ start: start - src + dst, len: end - start });
                    return new_ranges;
                } else {
                    new_ranges.push(Range{ start: start - src + dst, len: marker - start });
                    start = marker;
                }
            }
        }
        new_ranges.push(Range{ start: start, len: end - start });
        return new_ranges;
    }
}