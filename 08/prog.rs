const WIDTH: usize = 50;
const HEIGHT: usize = 6;

fn print_screen(screen: &[bool], width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            if screen[x + y*width] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn main() {
    let animate = std::env::args()
        .skip(1)
        .next()
        .and_then(|x| Some(x == "--animate"))
        .unwrap_or(false);

    let mut count = 0;
    let mut screen = [false; WIDTH * HEIGHT];

    let mut line = String::new();
    while let Ok(n) = std::io::stdin().read_line(&mut line) {
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
                            screen[i + j*WIDTH] = true;
                        }
                    }
                },
                "rotate" => {
                    assert_eq!("by", parts[3]);
                    let n: usize = parts[4].parse().unwrap();

                    match parts[1] {
                        "row" => {
                            let y: usize = parts[2].split("=").skip(1).next().unwrap().parse().unwrap();

                            let mut row = [false; WIDTH];
                            for i in 0..WIDTH {
                                let mut dest = i+n;
                                if dest >= WIDTH {
                                    dest -= WIDTH;
                                }
                                row[dest] = screen[i + y*WIDTH];
                            }
                            for i in 0..WIDTH {
                                screen[i + y*WIDTH] = row[i];
                            }
                        },
                        "column" => {
                            let x: usize = parts[2].split("=").skip(1).next().unwrap().parse().unwrap();

                            let mut column = [false; HEIGHT];
                            for i in 0..HEIGHT {
                                let mut dest = i+n;
                                if dest >= HEIGHT {
                                    dest -= HEIGHT;
                                }
                                column[dest] = screen[x + i*WIDTH];
                            }
                            for i in 0..HEIGHT {
                                screen[x + i*WIDTH] = column[i];
                            }
                        },
                        _ => panic!("unknown rotate: {}", parts[1])
                    }
                }
                _ => panic!("unknown instruction: {}", parts[0])
            }
        }

        if animate {
            print!("\x1b[2J\x1b[H"); // clear screen and set cursor to 0,0
            print_screen(&screen, WIDTH, HEIGHT);
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        line.clear();
    }
    if !animate {
        print_screen(&screen, WIDTH, HEIGHT);
    }
    for i in 0..WIDTH*HEIGHT {
        if screen[i] {
            count += 1;
        }
    }
    println!("{}", count);
}
