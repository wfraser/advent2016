extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;

use std::io::{self, Write};

const DIM: u8 = 4;

#[derive(Debug)]
struct Coord(u8, u8);

fn hash(key: &str, walk: &str) -> String {
    let mut md5 = Md5::new();
    md5.input(key.as_bytes());
    md5.input(walk.as_bytes());
    md5.result_str()
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    pub fn as_char(&self) -> char {
        match *self {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
        }
    }

    pub fn apply(&self, pos: &Coord) -> Coord {
        match *self {
            Direction::Up => Coord(pos.0, pos.1 - 1),
            Direction::Down => Coord(pos.0, pos.1 + 1),
            Direction::Left => Coord(pos.0 - 1, pos.1),
            Direction::Right => Coord(pos.0 + 1, pos.1),
        }
    }
}

fn unlocked(c: char) -> bool {
    match c {
        '0'...'9' | 'a' => true,
        'b'...'f' => false,
        _ => panic!("unexpected character in hash: {:?}", c)
    }
}

fn unlocked_directions(input: &str, pos: &Coord, walk: &str) -> Vec<Direction> {
    let mut directions = vec![];
    for (i,c) in hash(input, walk).chars().take(4).enumerate() {
        if unlocked(c) {
            match i {
                0 => if pos.1 > 0 { directions.push(Direction::Up); },
                1 => if pos.1 < DIM - 1 { directions.push(Direction::Down); },
                2 => if pos.0 > 0 { directions.push(Direction::Left); },
                3 => if pos.0 < DIM - 1 { directions.push(Direction::Right); },
                _ => unreachable!()
            }
        }
    }
    directions
}

fn walk(input: &str) -> String {
    let mut best_walk = String::new();

    #[derive(Debug)]
    struct Context {
        pos: Coord,
        walk: String,
        directions: Vec<Direction>,
    }

    let mut stack = Vec::<Context>::new();
    let mut current = Context {
        pos: Coord(0,0),
        walk: String::new(),
        directions: unlocked_directions(input, &Coord(0,0), ""),
    };

    loop {
        println!("{:?}", current);

        while current.directions.is_empty() {
            if stack.is_empty() {
                return best_walk;
            } else {
                current = stack.pop().unwrap();
                println!("popping stack");
                println!("{:?}", current);
            }
        }

        // do shit
        let direction = current.directions.pop().expect("empty direction vec");
        let newpos = direction.apply(&current.pos);
        if newpos.0 == DIM - 1 && newpos.1 == DIM - 1 {
            println!("success: {}", current.walk);
            best_walk = current.walk;
            if stack.is_empty() {
                return best_walk;
            } else {
                current = stack.pop().unwrap();
            }
        } else if best_walk.is_empty() || current.walk.len() < best_walk.len() {
            let mut newctx = Context {
                pos: newpos,
                walk: current.walk.clone(),
                directions: vec![],
            };
            newctx.walk.push(direction.as_char());
            newctx.directions = unlocked_directions(input, &newctx.pos, &newctx.walk);
            println!("{:?} takes us to {:?} with {:?}", direction, newctx.pos, newctx.directions);

            if !newctx.directions.is_empty() {
                stack.push(std::mem::replace(&mut current, newctx));
            }
        } else {
            println!("useless; turning back from {:?}", current);
            if stack.is_empty() {
                return best_walk;
            } else {
                current = stack.pop().unwrap();
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", walk(&input));
}
