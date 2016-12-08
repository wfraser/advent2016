const WIDTH: usize = 50;
const HEIGHT: usize = 6;

struct Matrix<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Copy> Matrix<T> {
    fn new(width: usize, height: usize, default: T) -> Matrix<T> {
        let mut data: Vec<T> = Vec::with_capacity(width * height);
        for _ in 0 .. width * height {
            data.push(default);
        }
        Matrix {
            data: data,
            width: width,
            height: height,
        }
    }

    fn get(&self, x: usize, y: usize) -> T {
        self.data[x + y*WIDTH]
    }

    fn set(&mut self, x: usize, y: usize, item: T) {
        self.data[x + y*WIDTH] = item;
    }

    fn set_rect(&mut self, width: usize, height: usize, item: T) {
        for i in 0..height {
            for j in 0..width {
                self.set(j, i, item);
            }
        }
    }

    fn rotate_row(&mut self, y: usize, n: usize) {
        let mut new_row = Vec::with_capacity(self.width);
        for i in 0..self.width {
            new_row.push(self.get((self.width + i - n) % self.width, y));
        }
        for i in 0..self.width {
            self.set(i, y, new_row[i]);
        }
    }

    fn rotate_column(&mut self, x: usize, n: usize) {
        let mut new_col = Vec::with_capacity(self.height);
        for i in 0..self.height {
            new_col.push(self.get(x, (self.height + i - n) % self.height));
        }
        for i in 0..self.height {
            self.set(x, i, new_col[i]);
        }
    }

    fn each<F: FnMut(&T)>(&self, mut f: F) {
        for item in self.data.iter() {
            f(item);
        }
    }
}

trait Printable {
    fn display_char(&self) -> char;
}

impl Printable for bool {
    fn display_char(&self) -> char {
        match *self {
            true => '#',
            false => '.',
        }
    }
}

impl<T: Printable + Copy> Matrix<T> {
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get(x, y).display_char());
            }
            println!("");
        }
    }
}

fn main() {
    let animate = std::env::args()
        .skip(1)
        .next()
        .and_then(|x| Some(x == "--animate"))
        .unwrap_or(false);

    let mut count = 0;
    let mut screen = Matrix::new(WIDTH, HEIGHT, false);

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

                    screen.set_rect(x, y, true);
                },
                "rotate" => {
                    assert_eq!("by", parts[3]);
                    let n: usize = parts[4].parse().unwrap();

                    match parts[1] {
                        "row" => {
                            let y: usize = parts[2].split("=").skip(1).next().unwrap().parse().unwrap();
                            screen.rotate_row(y, n);
                        },
                        "column" => {
                            let x: usize = parts[2].split("=").skip(1).next().unwrap().parse().unwrap();
                            screen.rotate_column(x, n);
                        },
                        _ => panic!("unknown rotate: {}", parts[1])
                    }
                }
                _ => panic!("unknown instruction: {}", parts[0])
            }
        }

        if animate {
            print!("\x1b[2J\x1b[H"); // clear screen and set cursor to 0,0
            screen.print();
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        line.clear();
    }
    if !animate {
        screen.print();
    }
    screen.each(|x| if *x { count += 1; });
    println!("{}", count);
}
