use std::collections::BTreeSet;
use std::io;

fn main() {
    let mut locations = BTreeSet::<(i64,i64)>::new();
    let mut x = 0i64;
    let mut y = 0i64;
    let mut direction = 0i64;
    let mut line = String::new();
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 { break; }
        for s in line.split(", ") {
            let turn: char;
            let dist: i64;
            let mut chars = s.chars();
            turn = chars.next().expect("1");
            dist = chars.as_str().trim().parse().expect("2");
            let mut mult = 1i64;
            match turn {
                'L' => {
                    direction += 1;
                    if direction == 4 {
                        direction = 0;
                    }
                },
                'R' => {
                    direction -= 1;
                    if direction == -1 {
                        direction = 3;
                    }
                },
                _ => panic!("invalid turn direction: {}", turn)
            }
            if direction == 2 || direction == 3 {
                mult = -1;
            }
            for _ in 0..dist {
                match direction {
                    0 | 2 => {
                        //x += mult * dist;
                        x += mult;
                    },
                    1 | 3 => {
                        //y += mult * dist;
                        y += mult;
                    },
                    _ => panic!("invalid direction: {}", direction)
                }
                if locations.contains(&(x,y)) {
                    println!("found it: {}, {}", x, y);
                    break;
                } else {
                    locations.insert((x, y));
                }
            }
            println!("{}: {} -> {},{}", turn, dist, x, y);
        }
        println!("{}", if x > 0 { x } else { -x } + if y > 0 { y } else { -y });
    }
}
