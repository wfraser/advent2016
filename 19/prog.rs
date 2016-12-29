use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Elf {
    gifts: u32,
    next: u32,
}

fn white_elephant(n: u32) -> u32 {
    let mut elves = HashMap::<u32, Elf>::with_capacity(n as usize);
    for i in 0..n {
        elves.insert(i, Elf {
            gifts: 1,
            next: (i+1)%n,
        });
    }

    let mut i = 0;
    loop {
        let (elf_gifts, next) = match elves.get(&i) {
            Some(elf) => (elf.gifts, elf.next),
            None => panic!("expected elf {}, but didn't find it", i)
        };

        if elf_gifts == 0 {
            panic!("unexpected elf with no gifts");
        } else if elf_gifts == n {
            return i;
        }

        let stolen = elves.remove(&next).expect("not found");
        let elf = elves.get_mut(&i).unwrap();
        //println!("{} steals {} from {}", i+1, stolen.gifts, next+1);
        elf.gifts += stolen.gifts;
        elf.next = stolen.next;
        i = elf.next;
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n: u32 = line.trim().parse().expect("invalid input");
    println!("elf {} wins", white_elephant(n) + 1);
}
