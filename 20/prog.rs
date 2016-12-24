use std::io;

struct BlockedIPs {
    pub ranges: Vec<(u32, u32)>,
}

impl BlockedIPs {
    pub fn new() -> BlockedIPs {
        BlockedIPs {
            ranges: vec![],
        }
    }

    pub fn insert(&mut self, lo: u32, hi: u32) {
        match self.ranges.binary_search_by_key(&lo, |pair| pair.0) {
            Ok(pos) => {
                let pair = self.ranges.get_mut(pos).unwrap();
                if pair.1 < hi {
                    pair.1 = hi;
                }
            },
            Err(pos) => {
                self.ranges.insert(pos, (lo, hi));
            }
        }
    }

    pub fn contains(&self, x: u32) -> Option<u32> {
        let mut max = None;
        for pair in self.ranges.iter() {
            if pair.0 > x {
                break;
            } else {
                let end = pair.1;
                if max.is_none() || max.unwrap() < end {
                    max = Some(end);
                }
            }
        }
        max
    }
}

fn main() {
    let mut blocked = BlockedIPs::new();
    let mut line = String::new();
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 { break; }
        {
            let parts = line.trim().split('-').collect::<Vec<&str>>();
            assert_eq!(2, parts.len());
            let lo: u32 = parts[0].parse().unwrap();
            let hi: u32 = parts[1].parse().unwrap();
            blocked.insert(lo, hi);
        }
        line.clear();
    }

    let mut count = 0;
    let mut first = true;
    let mut search = 0u32;
    loop {
        match blocked.contains(search) {
            Some(next) => {
                if next < search {
                    if first {
                        println!("{}", search);
                        first = false;
                    }
                    count += 1;
                    if search == std::u32::MAX {
                        break;
                    }
                    search += 1;
                } else {
                    if next == std::u32::MAX {
                        break;
                    }
                    search = next + 1;
                }
            },
            None => {
                if first {
                    println!("{}", search);
                    first = false;
                }
                count += 1;
                if search == std::u32::MAX {
                    break;
                }
                search += 1;
            }
        }
    }
    println!("{}", count);
}
