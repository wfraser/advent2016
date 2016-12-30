use std::io;

#[allow(dead_code)]
fn print(data: &[bool]) {
    for x in data {
        if *x {
            print!("1");
        } else {
            print!("0");
        }
    }
    println!("");
}

fn generate_data(initial: &[bool], len: usize) -> Vec<bool> {
    let mut a = initial.to_owned();
    while a.len() < len {
        let n = a.len();
        a.push(false);
        for i in 0 .. n {
            let x = !a[n-i-1];
            a.push(x);
        }
    }
    a
}

fn checksum(input: &[bool]) -> Vec<bool> {
    let mut out = Vec::<bool>::new();
    for pair in input.chunks(2) {
        out.push(pair[0] == pair[1]);
    }
    out
}

fn generate_checksum(input: &[bool], len: usize) -> Vec<bool> {
    use std::cmp::min;
    let mut out = checksum(&input[0..min(len, input.len())]);
    while out.len() % 2 == 0 {
        out = checksum(&out);
    }
    out
}

fn solve(input: &[bool], len: usize) -> String {
    let data = generate_data(input, len);
    let ck = generate_checksum(&data, len);
    ck.iter().map(|x| if *x { '1' } else { '0' }).collect()
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let input = line.trim().chars().map(|c| match c {
        '0' => false,
        '1' => true,
        _ => panic!("unexpected character {:?} in input", c)
    }).collect::<Vec<bool>>();

    println!("{}", solve(&input, 272));
    println!("{}", solve(&input, 35651584));
}
