// fn main1() {
//     let tests = vec![(7, 9), (15, 40), (30, 200)];
//     let inputs = vec![(46, 358), (68, 1054), (98, 1807), (66, 1080)];

//     let tests2 = vec![(71530, 940200)];
//     let inputs = vec![(46689866u64, 358105418071080u64)];

//     let mut results = Vec::new();
//     for (time, distance) in inputs {
//         let mut wins = 0;
//         for i in 1..time {
//             if i * (time - i) > distance {
//                 wins += 1;
//             }
//         }
//         results.push(wins);
//     }

//     println!("Awesome Possum: {}", results.iter().fold(1, |acc, x| acc * x));
// }

fn main() {
    // let test = (71530u64, 940200u64);
    let input = (46689866u64, 358105418071080u64);

    let (time, distance) = input;

    // find start
    let mut low = 0;
    let mut high = time / 2;
    while low <= high {
        let mid = low + ((high - low) / 2);
        if mid * (time - mid) > distance {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    println!("low: {}, {}", low, low * (time - low) > distance);
    println!("high: {}, {}", high, high * (time - high) > distance);

    let start = high;

    // find start
    let mut low = time / 2;
    let mut high = time;
    while low <= high {
        let mid = low + ((high - low) / 2);
        if mid * (time - mid) > distance {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    println!("low: {}, {}", low, low * (time - low) > distance);
    println!("high: {}, {}", high, high * (time - high) > distance);

    let end = high;
    println!("Awesome Possum: {}", end - start);
}
