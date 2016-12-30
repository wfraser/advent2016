use std::collections::{HashMap, VecDeque};
use std::io;

fn white_elephant(n: u32) -> u32 {
    #[derive(Debug, Clone)]
    struct Elf {
        next: u32,
    }

    let mut elves = HashMap::<u32, Elf>::with_capacity(n as usize);
    for i in 0..n {
        elves.insert(i, Elf {
            next: (i+1)%n,
        });
    }

    let mut i = 0;
    loop {
        if elves.len() == 1 {
            return i+1;
        }

        let elf = elves.get(&i).unwrap().clone();

        let stolen = elves.remove(&elf.next).expect("not found");
        let elf = elves.get_mut(&i).unwrap();
        //println!("{} steals from {}", i+1, next+1);
        elf.next = stolen.next;
        i = elf.next;
    }
}

fn white_elephant2(n: u32) -> u32 {
    let mut left = VecDeque::<u32>::new();
    let mut right = VecDeque::<u32>::new();
    for i in 0..n {
        let elf = i + 1;
        if i <= n / 2 {
            left.push_back(elf);
        } else {
            right.push_front(elf);
        }
    }

    while !left.is_empty() && !right.is_empty() {
        //println!("L: {:?}", left);
        //println!("R: {:?}", right);
        let _stolen = if left.len() > right.len() {
            left.pop_back().unwrap()
        } else {
            right.pop_back().unwrap()
        };
        //println!("removed {:?}", _stolen);
        right.push_front(left.pop_front().unwrap());
        left.push_back(right.pop_back().unwrap());
    }

    if left.is_empty() {
        right[0]
    } else {
        left[0]
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n: u32 = line.trim().parse().expect("invalid input");

    println!("elf {} wins", white_elephant(n));
    println!("elf {} wins", white_elephant2(n));
}
