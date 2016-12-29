struct Elf(u32);

fn white_elephant(n: usize) -> usize {
    let mut elves = (0..n).map(|_| Elf(1)).collect::<Vec<Elf>>();
    loop {
        for i in 0 .. n {
            let elf_gifts = elves.get(i).unwrap().0;
            if elf_gifts == 0 {
                println!("elf {} has none; skipping", i+1);
                continue;
            } else if elf_gifts as usize == n {
                return i;
            }
            let mut steal_from = (i + 1) % n;
            while steal_from != i {
                let stolen = std::mem::replace(&mut elves.get_mut(steal_from).unwrap().0, 0);
                if stolen == 0 {
                    steal_from = (steal_from + 1) % n;
                    continue;
                }
                println!("{} steals {} from {}", i+1, stolen, steal_from+1);
                elves.get_mut(i).unwrap().0 += stolen;
                break;
            }
        }
    }
}

fn main() {
    println!("elf {} wins", white_elephant(5) + 1);
}
