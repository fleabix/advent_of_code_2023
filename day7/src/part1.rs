use std::{path::Path, fs::File, io::Read, cmp::Ordering};
use nom;

fn main1() {
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

    let (_, mut games) = nom::multi::separated_list1(
        nom::character::complete::line_ending, 
        Game::parse
    )(&file_contents).unwrap();

    games.sort_by(|a, b| a.cmp(&b));

    let mut total = 0;
    let mut i = 1;
    for g in games {
        total += g.bid * i;
        i += 1;
    }

    println!("Total: {}", total);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[derive(Debug)]
struct Game {
    cards : Vec<usize>,
    bid : u64,
    hand : HandType,
}

impl Game {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        let (input, cards) = nom::combinator::map(
            nom::multi::many1(nom::character::complete::none_of(" ")),
            |v| {
                v.iter().map(|c| {
                    match *c {
                        '2' => 1,
                        '3' => 2,
                        '4' => 3,
                        '5' => 4,
                        '6' => 5,
                        '7' => 6,
                        '8' => 7,
                        '9' => 8,
                        'T' => 9,
                        'J' => 10,
                        'Q' => 11,
                        'K' => 12,
                        'A' => 13,
                        _ => panic!("what did you give me"),
                    }
                }
            ).collect::<Vec<usize>>()
            }
        )(input)?;
        let (input, _) = nom::character::complete::multispace1(input)?;
        let (input, bid) = nom::character::complete::u64(input)?;

        let mut hand = HandType::HighCard;
        let mut radix = vec![0u8;14];
        for n in &cards {
            let n = *n;
            let count = radix[n] + 1;
            radix[n] = count;
            hand = match count {
                2 => {
                    match hand {
                        HandType::HighCard => HandType::OnePair,
                        HandType::OnePair => HandType::TwoPair,
                        HandType::ThreeOfAKind => HandType::FullHouse,
                        _ => panic!("lol what hand: {:?}", hand)
                    }
                }
                3 => {
                    match hand {
                        HandType::OnePair => HandType::ThreeOfAKind,
                        HandType::TwoPair => HandType::FullHouse,
                        _ => panic!("lol what hand: {:?}", hand)
                    }
                }
                4 => {
                    match hand {
                        HandType::ThreeOfAKind => HandType::FourOfAKind,
                        _ => panic!("lol what hand: {:?}", hand)
                    }
                }
                5 => {
                    match hand {
                        HandType::FourOfAKind => HandType::FiveOfAKind,
                        _ => panic!("lol what hand: {:?}", hand)
                    }
                }
                _ => hand,
            };
        }

        Ok((input, Self { cards, bid, hand }))
    }

    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand == other.hand {
            self.cards.cmp(&other.cards)
        } else {
            self.hand.cmp(&other.hand)
        }
    }
}