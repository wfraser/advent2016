use std::collections::BTreeMap;
use std::io;

#[derive(Debug)]
enum Value {
    Register(String),
    Literal(i64),
}

impl Value {
    pub fn value(&self, regs: &mut BTreeMap<String, i64>) -> i64 {
        match self {
            &Value::Register(ref name) => regs[name],
            &Value::Literal(n) => n,
        }
    }

    pub fn parse(s: String) -> Value {
        if let Ok(n) = s.parse::<i64>() {
            Value::Literal(n)
        } else {
            Value::Register(s)
        }
    }
}

trait Instruction {
    fn execute(&self, regs: &mut BTreeMap<String, i64>) -> isize;
}

#[derive(Debug)]
struct CpyInstr {
    value: Value,
    dest_register: String,
}

impl CpyInstr {
    pub fn new(value: String, dest_register: String) -> CpyInstr {
        CpyInstr {
            value: Value::parse(value),
            dest_register: dest_register,
        }
    }
}

impl Instruction for CpyInstr {
    fn execute(&self, regs: &mut BTreeMap<String, i64>) -> isize {
        let v = self.value.value(regs);
        *regs.get_mut(self.dest_register.as_str()).unwrap() = v;
        1
    }
}

#[derive(Debug)]
struct SuccInstr {
    register: String,
    offset: i64,
}

impl SuccInstr {
    pub fn new(register: String, offset: i64) -> SuccInstr {
        SuccInstr {
            register: register,
            offset: offset,
        }
    }
}

impl Instruction for SuccInstr {
    fn execute(&self, regs: &mut BTreeMap<String, i64>) -> isize {
        *regs.get_mut(self.register.as_str()).unwrap() += self.offset;
        1
    }
}

#[derive(Debug)]
struct JnzInstr {
    value: Value,
    offset: isize,
}

impl JnzInstr {
    pub fn new(value: String, offset: String) -> JnzInstr {
        JnzInstr {
            value: Value::parse(value),
            offset: offset.parse().unwrap(),
        }
    }
}

impl Instruction for JnzInstr {
    fn execute(&self, regs: &mut BTreeMap<String, i64>) -> isize {
        let v = self.value.value(regs);
        if v != 0 {
            self.offset
        } else {
            1
        }
    }
}

fn run(instructions: &[Box<Instruction>], registers: &mut BTreeMap<String, i64>) {
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
            let opcode = words.next().unwrap();
            let arg1: String = words.next().unwrap().to_owned();
            let arg2: Option<String> = words.next().map(|x| x.to_owned());
            let instr: Box<Instruction> = match opcode {
                "cpy" => Box::new(CpyInstr::new(arg1, arg2.unwrap())),
                "inc" => Box::new(SuccInstr::new(arg1, 1)),
                "dec" => Box::new(SuccInstr::new(arg1, -1)),
                "jnz" => Box::new(JnzInstr::new(arg1, arg2.unwrap())),
                other => panic!("unknown opcode {:?}", other)
            };
            instructions.push(instr);
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

