#![feature(io)]

use std::io::{self, Read};

const INVALID_CHAR: char = '_';

struct Matrix<T> {
    pub data: Vec<T>,
    pub dim: usize,
}

impl<T> Matrix<T> {
    pub fn get(&self, pos: (usize, usize)) -> &T {
        self.data.get(pos.0 * self.dim + pos.1).unwrap()
    }
}

fn keypad_move(keypad: &Matrix<char>, current: (usize, usize), direction: char) -> (usize, usize) {
    let mut pos = current;
    match direction {
        'U' => if pos.0 > 0 { pos.0 -= 1; },
        'D' => if pos.0 + 1 < keypad.dim { pos.0 += 1; },
        'L' => if pos.1 > 0 { pos.1 -= 1; },
        'R' => if pos.1 + 1 < keypad.dim { pos.1 += 1; },
        _ => panic!("invalid direction {:?}", direction)
    }
    if keypad.get(pos) != &INVALID_CHAR {
        pos
    } else {
        current
    }
}

fn main() {
    let keypad1 = Matrix {
        data: vec![
            '1','2','3',
            '4','5','6',
            '7','8','9'],
        dim: 3,
    };
    let keypad2 = Matrix {
        data: vec![
            '_','_','1','_','_',
            '_','2','3','4','_',
            '5','6','7','8','9',
            '_','A','B','C','_',
            '_','_','D','_','_'],
        dim: 5,
    };

    let mut code1 = String::new();
    let mut code2 = String::new();

    let mut pos1 = (1,1);
    let mut pos2 = (2,0);
    let mut chars = io::stdin().chars();
    while let Some(Ok(c)) = chars.next() {
        if c == '\n' {
            code1.push(*keypad1.get(pos1));
            code2.push(*keypad2.get(pos2));
        } else {
            pos1 = keypad_move(&keypad1, pos1, c);
            pos2 = keypad_move(&keypad2, pos2, c);
        }
    }
    println!("{}", code1);
    println!("{}", code2);
}
