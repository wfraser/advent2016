use std::io;

struct Node {
    used: u32,
    avail: u32,
}

fn pair_is_viable(a: &Node, b: &Node) -> bool {
    a.used != 0
        && a.used <= b.avail
}

fn main() {
    let mut nodes = vec![];

    let mut line_number = 0;
    let mut line = String::new();
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 { break; }
        line_number += 1;

        if line_number > 2 {
            let words = line.split(|c| c == 'T' || c == ' ')
                            .filter(|s| !s.is_empty())
                            .collect::<Vec<&str>>();
            nodes.push(Node {
                used: words[2].parse().unwrap(),
                avail: words[3].parse().unwrap(),
            });
        }

        line.clear();
    }

    let mut num_viable_pairs = 0;
    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            if i != j && pair_is_viable(&nodes[i], &nodes[j]) {
                num_viable_pairs += 1;
            }
        }
    }

    println!("{}", num_viable_pairs);
}
