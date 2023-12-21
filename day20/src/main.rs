use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::Read,
    path::Path,
};

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

    let mut modules = HashMap::new();
    let mut conjunctions = HashMap::new();
    for line in file_contents.lines() {
        let mut split1 = line.split(" -> ");
        let name = split1.next().unwrap();
        let targets = split1
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.to_owned())
            .collect();
        let module = match name {
            "broadcaster" => Module {
                name: "broadcaster".to_string(),
                kind: ModuleType::Broadcaster,
                targets: targets,
            },
            s if s.starts_with("%") => Module {
                name: s[1..].to_string(),
                kind: ModuleType::FlipFlop(false),
                targets: targets,
            },
            s if s.starts_with("&") => {
                conjunctions.insert(s[1..].to_string(), Vec::new());
                Module {
                    name: s[1..].to_string(),
                    kind: ModuleType::Conjunction(HashMap::new()),
                    targets: targets,
                }
            }
            _ => unreachable!(),
        };
        modules.insert(module.name.clone(), module);
    }

    for (_, module) in &modules {
        for target in &module.targets {
            if conjunctions.contains_key(target) {
                conjunctions.get_mut(target).unwrap().push(module.name.clone());
            }
        }
    }
    for (key, sources) in conjunctions {
        let module = modules.get_mut(&key).unwrap();
        module.kind = ModuleType::Conjunction(sources.into_iter().map(|s|(s, false)).collect());
    }

    // let mut total_low = 0;
    // let mut total_high = 0;
    let mut pushes = 0u64;
    let mut elite_four = Vec::new();
    loop {
        pushes = pushes + 1;
        let mut high = 0u64;
        let mut low = 1u64;
        let mut work_queue = VecDeque::new();
        work_queue.push_back(Pulse::new(false, "button", "broadcaster"));
        while !work_queue.is_empty() {
            let pulse = work_queue.pop_front().unwrap();

            if pulse.high {
                match pulse.src.as_str() {
                    "hn" | "tg" | "lz" | "kh" => {
                        elite_four.push(pushes);
                        println!("{}: {}", pulse.dst, pushes);
                        
                        if elite_four.len() == 4 {
                            let total = elite_four.iter().fold(1u64, |t, x| t * x);
                            println!("Part 2: {}", total);
                            return;
                        }
                    },
                    _ => ()
                }
            }
            // {
            //     let p = match pulse.high {
            //         false => "low",
            //         true => "high",
            //     };
            //     println!("{} -{}-> {}", pulse.src, p, pulse.dst);
            // }

            let module = modules.get_mut(&pulse.dst);
            if module.is_none() {
                continue;
            }
            let module = module.unwrap();
            match &mut module.kind {
                ModuleType::Broadcaster => {
                    let emit = pulse.high;
                    for target in &module.targets {
                        if emit {
                            high = high + 1;
                        } else {
                            low = low + 1;
                        }
                        work_queue.push_back(Pulse::new(emit, &module.name, target));
                    }
                }
                ModuleType::FlipFlop(state) => {
                    if !pulse.high {
                        *state = !*state;
                        let emit = *state;
                        for target in &module.targets {
                            if emit {
                                high = high + 1;
                            } else {
                                low = low + 1;
                            }
                            work_queue.push_back(Pulse::new(emit, &module.name, target));
                        }
                    }
                }
                ModuleType::Conjunction(states) => {
                    states.insert(pulse.src, pulse.high);
                    let emit = !states.iter().all(|(_, high)| *high);
                    for target in &module.targets {
                        if emit {
                            high = high + 1;
                        } else {
                            low = low + 1;
                        }
                        work_queue.push_back(Pulse::new(emit, &module.name, target));
                    }
                }
            };
        }
        // println!("Part 1: {} {}", high, low);
        // total_low = total_low + low;
        // total_high = total_high + high;
    }
    // println!("Part 1: {}", total_high * total_low);
}

struct Module {
    name: String,
    kind: ModuleType,
    targets: Vec<String>,
}

enum ModuleType {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}

struct Pulse {
    src: String,
    dst: String,
    high: bool,
}

impl Pulse {
    fn new(high: bool, src: &str, dst: &str) -> Self {
        Self {
            high,
            src: src.to_owned(),
            dst: dst.to_owned(),
        }
    }
}
