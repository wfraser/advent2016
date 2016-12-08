use std::io;

fn main() {
    let mut count = 0;
    let mut screen = [[false;6];50];

    let mut line = String::new();
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 { break; }

        {
            let parts = line.trim().split(" ").collect::<Vec<&str>>();

            match parts[0] {
                "rect" => {
                    let mut coords = parts[1].split("x");
                    let x: usize = coords.next().unwrap().parse().unwrap();
                    let y: usize = coords.next().unwrap().parse().unwrap();

                    for i in 0..x {
                        for j in 0..y {
                            screen[i][j] = true;
                        }
                    }
                },
                "rotate" => {
                    assert_eq!("by", parts[3]);
                    match parts[1] {
                        "row" => {
                            let y: usize = parts[2].split("=").skip(1).next().unwrap().parse().unwrap();
                            let n: usize = parts[4].parse().unwrap();

                            let mut row = [false;50];
                            for i in 0..50 {
                                let mut dest = i+n;
                                if dest >= 50 {
                                    dest -= 50;
                                }
                                row[dest] = screen[i][y];
                            }
                            for i in 0..50 {
                                screen[i][y] = row[i];
                            }
                        },
                        "column" => {
                            let x: usize = parts[2].split("=").skip(1).next().unwrap().parse().unwrap();
                            let n: usize = parts[4].parse().unwrap();

                            let mut column = [false;6];
                            for i in 0..6 {
                                let mut dest = i+n;
                                if dest >= 6 {
                                    dest -= 6;
                                }
                                column[dest] = screen[x][i];
                            }
                            for i in 0..6 {
                                screen[x][i] = column[i];
                            }
                        },
                        _ => panic!("unknown rotate: {}", parts[1])
                    }
                }
                _ => panic!("unknown instruction: {}", parts[0])
            }
        }

        line.clear();
    }
    for y in 0..6 {
        for x in 0..50 {
            if screen[x][y] {
                print!("#");
                count += 1;
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("{}", count);
}
