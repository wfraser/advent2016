use std::io;

struct Elf(u32);

fn white_elephant(n: u32) -> u32 {
    let mut elves = (0..n).map(|_| Elf(1)).collect::<Vec<Elf>>();
    loop {
        for i in 0 .. n as usize {
            let elf_gifts = elves.get(i).unwrap().0;
            if elf_gifts == 0 {
                //println!("elf {} has none; skipping", i+1);
                continue;
            } else if elf_gifts == n {
                return i as u32;
            }
            let mut steal_from = (i + 1) % (n as usize);
            while steal_from != i {
                let stolen = std::mem::replace(&mut elves.get_mut(steal_from).unwrap().0, 0);
                if stolen == 0 {
                    steal_from = (steal_from + 1) % (n as usize);
                    continue;
                }
                //println!("{} steals {} from {}", i+1, stolen, steal_from+1);
                elves.get_mut(i).unwrap().0 += stolen;
                break;
            }
        }
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let n: u32 = line.trim().parse().expect("invalid input");
    println!("elf {} wins", white_elephant(n) + 1);
}
