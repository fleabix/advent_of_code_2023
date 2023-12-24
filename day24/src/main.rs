use std::{fs::File, io::Read, path::Path};

use z3::ast::{Ast, Int};
use z3::{Config, Context, Solver};

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

    let mut storm = Vec::new();
    for line in file_contents.lines() {
        let mut split1 = line.split(" @ ");
        let mut p_iter = split1
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.parse::<i64>().unwrap());
        let p = Point {
            x: p_iter.next().unwrap(),
            y: p_iter.next().unwrap(),
            z: p_iter.next().unwrap(),
        };
        let mut v_iter = split1
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.parse::<i64>().unwrap());
        let v = Point {
            x: v_iter.next().unwrap(),
            y: v_iter.next().unwrap(),
            z: v_iter.next().unwrap(),
        };
        storm.push(Hail { p, v });
    }

    let mut part1 = 0;
    for i in 0..storm.len() {
        for j in i + 1..storm.len() {
            let line1 = storm[i];
            let line2 = storm[j];

            let a1 = line1.v.y;
            let b1 = -line1.v.x;
            let c1 = a1 * line1.p.x + b1 * line1.p.y;

            let a2 = line2.v.y;
            let b2 = -line2.v.x;
            let c2 = a2 * line2.p.x + b2 * line2.p.y;

            let determinant = a1 * b2 - a2 * b1;
            if determinant == 0 {
                continue;
            } else {
                let x = (b2 * c1 - b1 * c2) / determinant;
                let y = (a1 * c2 - a2 * c1) / determinant;
                if x >= 200000000000000 && x <= 400000000000000 && y >= 200000000000000 && y <= 400000000000000 {
                    if (x - line1.p.x) / line1.v.x > 0
                        && (y - line1.p.y) / line1.v.y > 0
                        && (x - line2.p.x) / line2.v.x > 0
                        && (y - line2.p.y) / line2.v.y > 0
                    {
                        part1 = part1 + 1;
                    }
                }
            }
        }
    }

    println!("Part 1: {}", part1);

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let px = Int::new_const(&ctx, "px");
    let py = Int::new_const(&ctx, "py");
    let pz = Int::new_const(&ctx, "pz");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    for hailstone in storm {
        let pxn = Int::from_i64(&ctx, hailstone.p.x);
        let pyn = Int::from_i64(&ctx, hailstone.p.y);
        let pzn = Int::from_i64(&ctx, hailstone.p.z);
        let vxn = Int::from_i64(&ctx, hailstone.v.x);
        let vyn = Int::from_i64(&ctx, hailstone.v.y);
        let vzn = Int::from_i64(&ctx, hailstone.v.z);
        let tn = Int::fresh_const(&ctx, "t");

        solver.assert(&(&pxn + &vxn * &tn)._eq(&(&px + &vx * &tn)));
        solver.assert(&(&pyn + &vyn * &tn)._eq(&(&py + &vy * &tn)));
        solver.assert(&(&pzn + &vzn * &tn)._eq(&(&pz + &vz * &tn)));
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&px).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&py).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&pz).unwrap().as_i64().unwrap();

    println!("Part 2: {}", x + y + z);

}


#[derive(Debug, Clone, Copy)]
struct Hail {
    p: Point,
    v: Point,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}
