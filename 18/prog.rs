use std::io;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Safe,
    Trap,
}

impl Tile {
    pub fn is_trap(&self) -> bool {
        match *self {
            Tile::Trap => true,
            _ => false
        }
    }
}

#[allow(dead_code)]
fn print_row(row: &[Tile]) {
    for tile in row {
        match tile {
            &Tile::Trap => print!("^"),
            &Tile::Safe => print!("."),
        }
    }
    println!("");
}

fn next_tile(pos: usize, prev_row: &[Tile]) -> Tile {
    let left = if pos == 0 { Tile::Safe } else { prev_row[pos - 1] };
    let center = prev_row[pos];
    let right = if prev_row.len() == pos + 1 { Tile::Safe } else { prev_row[pos + 1] };
    if (left.is_trap() && center.is_trap() && !right.is_trap())
            || (center.is_trap() && right.is_trap() && !left.is_trap())
            || (left.is_trap() && !center.is_trap() && !right.is_trap())
            || (right.is_trap() && !center.is_trap() && !left.is_trap()) {
        return Tile::Trap;
    } else {
        return Tile::Safe;
    }
}

fn gen_tiles(n: u32, mut row: Vec<Tile>) -> u32 {
    let mut sum = 0;
    for i in 0..n {
        for tile in row.iter() {
            if !tile.is_trap() {
                sum += 1;
            }
        }

        if i < n-1 {
            let mut new_row = vec![];
            for pos in 0..row.len() {
                new_row.push(next_tile(pos, &row));
            }
            std::mem::replace(&mut row, new_row);
        }
    }
    sum
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    if line.ends_with("\n") {
        line.pop();
    }

    let row = line.chars().map(|c| {
        match c {
            '^' => Tile::Trap,
            '.' => Tile::Safe,
            _ => panic!("invalid input {:?}", c)
        }
    }).collect::<Vec<Tile>>();

    //print!("row 0: ");
    //print_row(&row);

    println!("40 rows: {}", gen_tiles(40, row.clone()));
    println!("400000 rows: {}", gen_tiles(400000, row));
}
