extern crate crypto;
use crypto::digest::Digest;
use crypto::md5::Md5;
use std::io::{self, Write};

fn hash(key: &str, n: u64) -> String {
    let mut md5 = Md5::new();
    md5.input(key.as_bytes());
    md5.input(format!("{}", n).as_bytes());
    md5.result_str()
}

fn main() {
    let mut prefix = String::new();
    io::stdin().read_line(&mut prefix).unwrap();
    let mut n = 0u64;
    let mut answer1 = String::new();
    let mut answer2 = vec![' ';8];
    let mut answer2_letters = 0;
    loop {
        let out = hash(prefix.trim(), n);
        if out.starts_with("00000") {
            println!("{}", out);

            let mut chars = out.chars().skip(5);
            let char5 = chars.next().unwrap();
            let char6 = chars.next().unwrap();

            if answer1.len() < 8 {
                answer1.push(char5);
            }

            if let Ok(pos) = format!("{}", char5).parse::<usize>() {
                if answer2.get(pos) == Some(&' ') {
                    answer2[pos] = char6;
                    answer2_letters += 1;
                    if answer2_letters == 8 {
                        break;
                    }
                }
            }
        }
        if n % 100000 == 0 {
            print!(".");
            io::stdout().flush().unwrap();
        }
        n += 1;
    }
    println!("{}", answer1);
    let answer2_str: String = answer2.into_iter().collect();
    println!("{}", answer2_str);
}
