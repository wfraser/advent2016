use std::collections::BTreeMap;
use std::io;

#[derive(Debug)]
enum Opcode { CPY, INC, DEC, JNZ }

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    arg1: Option<String>,
    arg2: Option<String>,
}

fn value(s: &str, regs: &BTreeMap<String, i64>) -> i64 {
    if let Ok(n) = s.parse() {
        n
    } else {
        regs[s]
    }
}

impl Instruction {
    fn execute(&self, regs: &mut BTreeMap<String, i64>) -> isize {
        match self.opcode {
            Opcode::CPY => {
                let v = value(self.arg1.as_ref().unwrap(), regs);
                *regs.get_mut(self.arg2.as_ref().unwrap()).unwrap() = v;
                1
            },
            Opcode::INC => {
                *regs.get_mut(self.arg1.as_ref().unwrap()).unwrap() += 1;
                1
            },
            Opcode::DEC => {
                *regs.get_mut(self.arg1.as_ref().unwrap()).unwrap() -= 1;
                1
            },
            Opcode::JNZ => {
                let v = value(self.arg1.as_ref().unwrap(), regs);
                if v != 0 {
                    self.arg2.as_ref().unwrap().parse().unwrap()
                } else {
                    1
                }
            }
        }
    }
}

fn run(instructions: &[Instruction], registers: &mut BTreeMap<String, i64>) {
    let mut i = 0isize;
    loop {
        let offset = instructions[i as usize].execute(registers);
        i += offset;
        if (i as usize) >= instructions.len() {
            break;
        }
    }
}

fn main() {
    let mut instructions = vec![];
    let mut line = String::new();
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 { break; }
        {
            let mut words = line.trim().split(" ");
            let opcode = match words.next().unwrap() {
                "cpy" => Opcode::CPY,
                "inc" => Opcode::INC,
                "dec" => Opcode::DEC,
                "jnz" => Opcode::JNZ,
                other => panic!("unknown opcode {:?}", other)
            };
            let arg1 = words.next().map(|s| s.to_owned());
            let arg2 = words.next().map(|s| s.to_owned());
            instructions.push(Instruction {
                opcode: opcode,
                arg1: arg1,
                arg2: arg2,
            });
        }
        line.clear();
    }

    let mut registers = BTreeMap::new();
    registers.insert("a".to_owned(), 0);
    registers.insert("b".to_owned(), 0);
    registers.insert("c".to_owned(), 0);
    registers.insert("d".to_owned(), 0);

    run(&instructions, &mut registers);
    println!("{}", registers["a"]);

    registers.insert("a".to_owned(), 0);
    registers.insert("b".to_owned(), 0);
    registers.insert("c".to_owned(), 1);
    registers.insert("d".to_owned(), 0);

    run(&instructions, &mut registers);
    println!("{}", registers["a"]);
}

