use std::io;

fn has_abba(chars: &[char]) -> bool {
    for window in chars.windows(4) {
        if window[0] == window[3] && window[1] == window[2] && window[0] != window[1] {
            return true;
        }
    }
    false
}

fn has_aba_bab(outsides: &[Vec<char>], insides: &[Vec<char>]) -> bool {
    for chars in outsides {
        for window in chars.windows(3) {
            if window[0] == window[2] && window[0] != window[1] {
                if has_bab(insides, window[0], window[1]) {
                    return true;
                }
            }
        }
    }
    false
}

fn has_bab(insides: &[Vec<char>], a: char, b: char) -> bool {
    for chars in insides {
        for window in chars.windows(3) {
            if window[0] == b && window[1] == a && window[2] == b {
                return true;
            }
        }
    }
    false
}

fn main() {
    let mut count1 = 0;
    let mut count2 = 0;
    let mut line = String::new();
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 { break; }

        let mut in_brackets = false;
        let mut insides  = Vec::<Vec<char>>::new();
        let mut outsides = Vec::<Vec<char>>::new();
        let mut buf = Vec::<char>::new();
        for c in line.chars() {
            if c == '[' || c == '\n' {
                assert!(!in_brackets);
                in_brackets = true;
                outsides.push(std::mem::replace(&mut buf, vec![]));
            } else if c == ']' {
                assert!(in_brackets);
                in_brackets = false;
                insides.push(std::mem::replace(&mut buf, vec![]));
            } else {
                buf.push(c);
            }
        }

        let mut part1_match = false;
        for chars in &outsides {
            if has_abba(&chars) {
                part1_match = true;
                break;
            }
        }
        if part1_match {
            for chars in &insides {
                if has_abba(&chars) {
                    part1_match = false;
                    break;
                }
            }
        }
        if part1_match {
            count1 += 1;
        }

        if has_aba_bab(&outsides, &insides) {
            count2 += 1;
        }

        line.clear();
    }
    println!("{}", count1);
    println!("{}", count2);
}
