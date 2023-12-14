use nom;
use std::{collections::HashSet, fs::File, io::Read, path::Path};

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

    let (_, scratchcards) = Scratchcards::parse(&s).unwrap();

    let mut card_piles = vec![1u64; scratchcards.cards.len().try_into().unwrap()];

    let mut i = 0;
    for card in scratchcards.cards {
        let mut winning_set = HashSet::new();
        for n in card.winning_numbers {
            winning_set.insert(n);
        }
        let mut matches = 0;
        for n in card.numbers_on_card {
            if winning_set.contains(&n) {
                matches += 1;
            }
        }
        if matches > 0 {
            for n in 1..=matches {
                card_piles[i + n] += card_piles[i];
            }
        }
        i += 1;
    }

    let total: u64 = card_piles.iter().sum();

    println!("Total: {}", total)
}

struct Scratchcards {
    cards: Vec<Card>,
}

impl Scratchcards {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let (input, cards) =
            nom::multi::separated_list1(nom::character::complete::line_ending, Card::parse)(input)?;
        Ok((input, Self { cards }))
    }
}

struct Card {
    winning_numbers: Vec<u64>,
    numbers_on_card: Vec<u64>,
}

impl Card {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let (input, _) = nom::bytes::complete::tag("Card")(input)?;
        let (input, _) = nom::character::complete::space1(input)?;
        let (input, _) = nom::character::complete::u64(input)?;
        let (input, _) = nom::character::complete::char(':')(input)?;
        let (input, _) = nom::character::complete::space1(input)?;
        let (input, winning_numbers) = nom::multi::separated_list1(
            nom::character::complete::space1,
            nom::character::complete::u64,
        )(input)?;
        let (input, _) = nom::sequence::delimited(
            nom::character::complete::space1,
            nom::character::complete::char('|'),
            nom::character::complete::space1,
        )(input)?;
        let (input, numbers_on_card) = nom::multi::separated_list1(
            nom::character::complete::space1,
            nom::character::complete::u64,
        )(input)?;
        Ok((
            input,
            Self {
                winning_numbers,
                numbers_on_card,
            },
        ))
    }
}
