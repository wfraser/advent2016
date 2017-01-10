extern crate linked_list;
use linked_list::LinkedList;

fn generate_elves(n: u32) -> LinkedList<u32> {
    let mut elves = LinkedList::<u32>::new();
    for i in 1 .. n+1 {
        elves.push_back(i);
    }
    elves
}

fn white_elephant(n: u32) -> u32 {
    let mut elves = generate_elves(n);
    {
        let mut cursor = elves.cursor();
        cursor.seek_forward(1);
        
        for _ in 0 .. n - 1 {
            // steal from the next elf
            if cursor.peek_next().is_none() {
                // wrap around
                cursor.seek_forward(1);
            }
            cursor.remove();

            // advance to the next elf
            if cursor.peek_next().is_none() {
                cursor.seek_forward(2);
            } else {
                cursor.seek_forward(1);
            }
        }
    }
    elves.front().cloned().unwrap()
}

fn white_elephant2(n: u32) -> u32 {
    let mut elves = generate_elves(n);

    {
        let mut cursor = elves.cursor();
        // seek to halfway around the circle to start.
        cursor.seek_forward(n as usize / 2);

        for i in 0 .. n - 1 {
            if cursor.peek_next().is_none() {
                cursor.seek_forward(1);
            }
            cursor.remove().unwrap();
            
            // if this removal didn't make the half-circle size shrink, advance by one
            let last_size = (n - i) / 2;
            let this_size = (n - i - 1) / 2;
            if last_size == this_size {
                if cursor.peek_next().is_none() {
                    cursor.seek_forward(2);
                } else {
                    cursor.seek_forward(1);
                }
            }
        }
    }
    elves.front().cloned().unwrap() 
}

fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n: u32 = line.trim().parse().expect("invalid input");

    println!("elf {} wins", white_elephant(n));
    println!("elf {} wins", white_elephant2(n));
}
