use std::io;

#[derive(Debug)]
struct Registers {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}

impl Registers {
    pub fn get(&self, name: &str) -> i64 {
        match name {
            "a" => self.a,
            "b" => self.b,
            "c" => self.c,
            "d" => self.d,
            _ => panic!("invalid register name {:?}", name)
        }
    }

    pub fn get_mut(&mut self, name: &str) -> &mut i64 {
        match name {
            "a" => &mut self.a,
            "b" => &mut self.b,
            "c" => &mut self.c,
            "d" => &mut self.d,
            _ => panic!("invalid register name {:?}", name)
        }
    }
}

#[derive(Debug)]
enum Value {
    Register(String),
    Literal(i64),
}

impl Value {
    pub fn value(&self, regs: &mut Registers) -> i64 {
        match self {
            &Value::Register(ref name) => regs.get(name),
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
    fn execute(&self, regs: &mut Registers) -> isize;
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
    fn execute(&self, regs: &mut Registers) -> isize {
        let v = self.value.value(regs);
        *regs.get_mut(self.dest_register.as_str()) = v;
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
    fn execute(&self, regs: &mut Registers) -> isize {
        *regs.get_mut(self.register.as_str()) += self.offset;
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
    fn execute(&self, regs: &mut Registers) -> isize {
        let v = self.value.value(regs);
        if v != 0 {
            self.offset
        } else {
            1
        }
    }
}

fn run(instructions: &[Box<Instruction>], registers: &mut Registers) {
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

    let mut registers = Registers {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    };
    
    run(&instructions, &mut registers);
    println!("{}", registers.a);

    registers = Registers {
        a: 0,
        b: 0,
        c: 1,
        d: 0,
    };

    run(&instructions, &mut registers);
    println!("{}", registers.a);
}
