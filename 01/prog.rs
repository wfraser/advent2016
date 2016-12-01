use std::collections::BTreeSet;
use std::io;

struct State {
    pub x: i64,
    pub y: i64,
    pub direction: u8,
}

impl State {
    pub fn new() -> State {
        State {
            x: 0,
            y: 0,
            direction: 0, 
        }
    }

    pub fn turn(&mut self, direction: char) {
        match direction {
            'L' => {
                self.direction += 1;
                if self.direction == 4 {
                    self.direction = 0;
                }
            },
            'R' => {
                if self.direction == 0 {
                    self.direction = 3;
                } else {
                    self.direction -= 1;
                }
            },
            _ => panic!("invalid turn direction {:?}", direction)
        }
    }

    pub fn walk(&mut self, distance: i64) {
        match self.direction {
            0 => { self.y += distance },
            1 => { self.x += distance },
            2 => { self.y -= distance },
            3 => { self.x -= distance },
            _ => unreachable!(),
        }
    }           
}

fn main() {
    let mut locations = BTreeSet::<(i64,i64)>::new();
    let mut state = State::new();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("read error");
    let mut visited_twice = false;
    for instruction in input.split(", ") {
        let turn: char;
        let dist: i64;

        let mut chars = instruction.chars();
        turn = chars.next().expect("error getting turn direction");
        dist = chars.as_str().trim().parse().expect("error getting distance");

        state.turn(turn);
        for _ in 0 .. dist {
            state.walk(1);

            if !visited_twice && locations.contains(&(state.x, state.y)) {
                visited_twice = true;
                println!("found it: {}, {} ({} away)", state.x, state.y, state.x.abs() +
                         state.y.abs());
            } else {
                locations.insert((state.x, state.y));
            }
        }
    }
    println!("final location: {}, {} ({} away)", state.x, state.y, state.x.abs() + state.y.abs());
}
