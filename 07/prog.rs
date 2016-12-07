use std::io;

fn check(chars: &[char]) -> bool {
    for window in chars.windows(4) {
        if window[0] == window[3] && window[1] == window[2] && window[0] != window[1] {
            return true;
        }
    }
    false
}

fn main() {
    let mut count1 = 0;
    let mut line = String::new();
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 { break; }

        let mut in_brackets = false;
        let mut inside = Vec::new();
        let mut outside = Vec::new();
        let mut valid = true;
        let mut abbas_outside = false;
        for c in line.chars() {
            if c == '[' || c == '\n' {
                assert!(!in_brackets);
                in_brackets = true;
                if check(&outside) {
                    abbas_outside = true;
                }
                outside.clear();
            } else if c == ']' {
                assert!(in_brackets);
                in_brackets = false;
                if check(&inside) {
                    valid = false;
                    break;
                }
                inside.clear();
            } else if in_brackets {
                inside.push(c);
            } else {
                outside.push(c);
            }
        }

        if valid && abbas_outside {
            count1 += 1;
        }

        inside.clear();
        outside.clear();
        line.clear();
    }
    println!("{}", count1);
}
