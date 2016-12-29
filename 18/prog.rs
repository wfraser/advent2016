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

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    if line.ends_with("\n") {
        line.pop();
    }

    let mut row = line.chars().map(|c| {
        match c {
            '^' => Tile::Trap,
            '.' => Tile::Safe,
            _ => panic!("invalid input {:?}", c)
        }
    }).collect::<Vec<Tile>>();

    print!("row 0: ");
    print_row(&row);

    for i in 1..41 {
        let mut new_row = vec![];
        for pos in 0..row.len() {
            new_row.push(next_tile(pos, &row));
        }
        print!("row {}: ", i);
        print_row(&new_row);
        std::mem::replace(&mut row, new_row);
    }

    let mut sum = 0;
    for tile in row {
        if !tile.is_trap() {
            sum += 1;
        }
    }
    println!("{}", sum);
}
