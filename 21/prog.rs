#![feature(try_from)]

use std::convert::TryInto;
use std::error::Error;
use std::io;

#[derive(Debug, Clone, Copy)]
enum Op {
    SwapPosition(u32, u32),
    SwapLetter(char, char),
    RotateSteps(i32),
    RotateLetter(char),
    Reverse(u32, u32),
    Move(u32, u32),
}

fn swap<T: Copy>(x: &mut Vec<T>, a: usize, b: usize) {
    let tmp = x[a];
    x[a] = x[b];
    x[b] = tmp;
}

fn rotate<T: Copy>(x: &mut Vec<T>, n: isize) {
    let mut vec2 = Vec::with_capacity(x.len());
    let len = x.len() as isize;
    for i in 0..len {
        let mut pos = i + n;
        while pos >= len {
            pos -= len;
        }
        while pos < 0 {
            pos += len;
        }
        vec2.push(x[pos as usize]);
    }
    for i in 0..x.len() {
        x[i] = vec2[i];
    }
}

impl Op {
    fn run(&self, x: &mut Vec<char>, rev: bool) {
        match *self {
            Op::SwapPosition(src, dst) => {
                swap(x, src as usize, dst as usize);
            },
            Op::SwapLetter(c1, c2) => {
                let pos1 = x.iter().position(|c| c == &c1);
                let pos2 = x.iter().position(|c| c == &c2);
                if let (Some(pos1), Some(pos2)) = (pos1, pos2) {
                    swap(x, pos1, pos2);
                } else {
                    panic!("letter not found");
                }
            },
            Op::RotateSteps(mut num) => {
                if rev { num *= -1; }
                rotate(x, num as isize);
            },
            Op::RotateLetter(ch) => {
                let pos = x.iter().position(|c| c == &ch).unwrap();
                if rev {
                    //FIXME: not sure how to come up with the general pattern yet.
                    assert_eq!(8, x.len());
                    let n = match pos {
                        0 => 9,
                        1 => 1,
                        2 => 6,
                        3 => 2,
                        4 => 7,
                        5 => 3,
                        6 => 8,
                        7 => 4,
                        _ => unreachable!(),
                    };
                    rotate(x, n);
                } else {
                    let n = if pos >= 4 {
                        pos + 2
                    } else {
                        pos + 1
                    };
                    rotate(x, n as isize * -1);
                }
            },
            Op::Reverse(start_pos, end_pos) => {
                let mut mid = x.split_off(start_pos as usize);
                let mut end = mid.split_off(1 + end_pos as usize - start_pos as usize);
                for c in mid.iter().rev() {
                    x.push(*c);
                }
                x.append(&mut end);
            },
            Op::Move(mut pos1, mut pos2) => {
                if rev {
                    let tmp = pos1;
                    pos1 = pos2;
                    pos2 = tmp;
                }
                let c = x.remove(pos1 as usize);
                x.insert(pos2 as usize, c);
            }
        }
    }
}

#[derive(Debug)]
struct ParseErr(&'static str);

impl std::fmt::Display for ParseErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "parse error: {}", self.0)
    }
}

impl Error for ParseErr {
    fn description(&self) -> &'static str {
        "parse error"
    }
}

impl<'a> TryInto<Op> for &'a str {
    type Err = Box<Error>;
    fn try_into(self) -> Result<Op, Self::Err> {
        let words = self.split_whitespace().collect::<Vec<&str>>();
        if self.starts_with("swap position") {
            Ok(Op::SwapPosition(words[2].parse()?, words[5].parse()?))
        } else if self.starts_with("swap letter") {
            Ok(Op::SwapLetter(
                words[2].chars().next().ok_or(ParseErr("empty words[2]"))?,
                words[5].chars().next().ok_or(ParseErr("empty words[5]"))?))
        } else if self.starts_with("rotate ") {
            if words[1] == "left" {
                Ok(Op::RotateSteps(words[2].parse()?))
            } else if words[1] == "right" {
                Ok(Op::RotateSteps(words[2].parse::<i32>()? * -1))
            } else if words[1] == "based" {
                Ok(Op::RotateLetter(words[6].chars().next().ok_or(ParseErr("empty words[6]"))?))
            } else {
                Err(Box::new(ParseErr("unknown reverse operation")))
            }
        } else if self.starts_with("reverse") {
            Ok(Op::Reverse(words[2].parse()?, words[4].parse()?))
        } else if self.starts_with("move") {
            Ok(Op::Move(words[2].parse()?, words[5].parse()?))
        } else {
            Err(Box::new(ParseErr("unknown operation")))
        }
    }
}

fn main() {
    let mut ops = Vec::<Op>::new();
    let mut line = String::new();
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 { break; }
        match line.trim().try_into() {
            Ok(op) => ops.push(op),
            Err(e) => {
                panic!("error parsing {:?}: {}", line, e);
            }
        }
        line.clear();
    }

    let mut password = "abcdefgh".chars().collect::<Vec<char>>();
    for op in ops.iter() {
        //print!("{:?}: ", op);
        op.run(&mut password, false);
        //println!("{}", password.iter().cloned().collect::<String>());
    }
    println!("{}", password.into_iter().collect::<String>());

    password = "fbgdceah".chars().collect::<Vec<char>>();
    for op in ops.iter().rev() {
        op.run(&mut password, true);
    }
    println!("{}", password.into_iter().collect::<String>());
}
