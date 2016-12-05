use std::io;
use std::iter::FromIterator;

#[derive(Debug)]
struct RoomInfo {
    room: String,
    sector: u64, 
    checksum: String,
}

fn parse_line(line: &str) -> RoomInfo {
    let mut info = RoomInfo {
        room: String::new(),
        sector: 0,
        checksum: String::new(),
    };
    let mut v = Vec::<char>::new();
    let mut last_dash_pos = 0usize;
    let mut have_sector = false;
    for c in line.trim().chars() {
        match c {
            '-' => {
                v.push(c);
                last_dash_pos = v.len();
            },
            '[' => {
                assert!(!have_sector);
                let new = v.split_off(last_dash_pos);
                info.room = String::from_iter(v.drain(..));
                info.room.pop();
                let sector = String::from_iter(new.into_iter());
                info.sector = sector.parse().unwrap();
                have_sector = true;
            },
            ']' => {
                return info;
            },
            _ => {
                if !have_sector {
                    v.push(c);
                } else {
                    info.checksum.push(c);
                }
            },
        }
    }
    unreachable!();
}

fn make_checksum(s: &str) -> String {
    //use std::collections::btree_map::*;
    use std::cmp::Ordering;
   
    //let mut freq = BTreeMap::<char, u64>::new();
    let mut pairs = Vec::<(char, u64)>::new();

    for c in s.chars() {
        if c == '-' { continue; }
        /*
        match freq.entry(c) {
            Entry::Vacant(entry) => { entry.insert(1); },
            Entry::Occupied(entry) => *entry.into_mut() += 1,
        }
        */
        let found = match pairs.iter_mut().find(|pair| pair.0 == c) {
            Some(pair) => {
                pair.1 += 1;
                true
            },
            None => false
        };
        if !found {
            pairs.push((c, 1));
        }
    }

    //let mut pairs = freq.into_iter().collect::<Vec<(char, u64)>>();
    pairs.sort_by(|a, b| {
        let num_cmp = b.1.cmp(&a.1);
        if num_cmp == Ordering::Equal {
            a.0.cmp(&b.0)
        } else {
            num_cmp
        }
    });

    let mut checksum = String::new();
    for pair in pairs {
        checksum.push(pair.0);
        if checksum.len() == 5 {
            break;
        }
    }
    
    checksum
}

fn main() {
    let mut sum = 0;
    let mut line = String::new();
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 { break; }
        let info = parse_line(&line);

        let expected_checksum = make_checksum(&info.room);

        if expected_checksum == info.checksum {
            sum += info.sector;
        }
        line.clear();
    }

    println!("{}", sum);
}
