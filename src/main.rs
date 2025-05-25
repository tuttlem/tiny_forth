use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Push(i32),
    Add,
    Mul,
    Dup,
    Drop,
    Swap,
    Over,
    Rot,
    Nip,
    Tuck,
    TwoDup,
    TwoDrop,
    TwoSwap,
    Depth,
    Jump(isize),
    IfZero(isize),
    Call(usize),
    CallWord(String),
    Return,
    Halt,
}

#[derive(Debug)]
struct VM {
    stack: Vec<i32>,
    program: Vec<Instruction>,
    ip: usize,
    return_stack: Vec<usize>,
    dictionary: HashMap<String, usize>,
}

impl VM {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            stack: Vec::new(),
            program,
            ip: 0,
            return_stack: Vec::new(),
            dictionary: HashMap::new(),
        }
    }

    fn add_word(&mut self, name: &str, address: usize) {
        self.dictionary.insert(name.to_string(), address);
    }

    fn run(&mut self) {
        while self.ip < self.program.len() {
            match &self.program[self.ip] {
                Instruction::Push(value) => {
                    self.stack.push(*value);
                }
                Instruction::Add => {
                    let b = self.stack.pop().expect("Stack underflow on ADD");
                    let a = self.stack.pop().expect("Stack underflow on ADD");
                    self.stack.push(a + b);
                }
                Instruction::Mul => {
                    let b = self.stack.pop().expect("Stack underflow on MUL");
                    let a = self.stack.pop().expect("Stack underflow on MUL");
                    self.stack.push(a * b);
                }
                Instruction::Dup => {
                    let top = *self.stack.last().expect("Stack underflow on DUP");
                    self.stack.push(top);
                }
                Instruction::Drop => {
                    self.stack.pop().expect("Stack underflow on DROP");
                }
                Instruction::Swap => {
                    let b = self.stack.pop().expect("Stack underflow on SWAP");
                    let a = self.stack.pop().expect("Stack underflow on SWAP");
                    self.stack.push(b);
                    self.stack.push(a);
                }
                Instruction::Over => {
                    if self.stack.len() < 2 {
                        panic!("Stack underflow on OVER");
                    }
                    let val = self.stack[self.stack.len() - 2];
                    self.stack.push(val);
                }
                Instruction::Rot => {
                    if self.stack.len() < 3 {
                        panic!("Stack underflow on ROT");
                    }
                    let c = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(b);
                    self.stack.push(c);
                    self.stack.push(a);
                }
                Instruction::Nip => {
                    if self.stack.len() < 2 {
                        panic!("Stack underflow on NIP");
                    }
                    let top = self.stack.pop().unwrap();
                    self.stack.pop(); // discard second
                    self.stack.push(top);
                }
                Instruction::Tuck => {
                    if self.stack.len() < 2 {
                        panic!("Stack underflow on TUCK");
                    }
                    let top = *self.stack.last().unwrap();
                    let second = self.stack[self.stack.len() - 2];
                    self.stack.insert(self.stack.len() - 2, top);
                }
                Instruction::TwoDup => {
                    if self.stack.len() < 2 {
                        panic!("Stack underflow on 2DUP");
                    }
                    let len = self.stack.len();
                    self.stack.push(self.stack[len - 2]);
                    self.stack.push(self.stack[len - 1]);
                }
                Instruction::TwoDrop => {
                    if self.stack.len() < 2 {
                        panic!("Stack underflow on 2DROP");
                    }
                    self.stack.pop();
                    self.stack.pop();
                }
                Instruction::TwoSwap => {
                    if self.stack.len() < 4 {
                        panic!("Stack underflow on 2SWAP");
                    }
                    let d = self.stack.pop().unwrap();
                    let c = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(c);
                    self.stack.push(d);
                    self.stack.push(a);
                    self.stack.push(b);
                }
                Instruction::Depth => {
                    let depth = self.stack.len() as i32;
                    self.stack.push(depth);
                }
                Instruction::Call(addr) => {
                    self.return_stack.push(self.ip + 1);
                    self.ip = *addr;
                    continue;
                }
                Instruction::CallWord(name) => {
                    let addr = self.dictionary.get(name)
                        .expect(&format!("Unknown word: {}", name));
                    self.return_stack.push(self.ip + 1);
                    self.ip = *addr;
                    continue;
                }
                Instruction::Return => {
                    let ret = self.return_stack.pop().expect("Return stack underflow");
                    self.ip = ret;
                    continue;
                }
                Instruction::IfZero(offset) => {
                    let cond = self.stack.pop().expect("Stack underflow on IFZERO");
                    if cond == 0 {
                        self.ip = ((self.ip as isize) + offset) as usize;
                        continue; // skip ip += 1
                    }
                }
                Instruction::Jump(offset) => {
                    self.ip = ((self.ip as isize) + offset) as usize;
                    continue;
                }
                Instruction::Halt => break,
            }


            self.ip += 1;
        }
    }

}

struct Parser {
    main: Vec<Instruction>,
    definitions: Vec<Instruction>,
    dictionary: HashMap<String, usize>,
}

impl Parser {
    fn new() -> Self {
        Self {
            main: Vec::new(),
            definitions: Vec::new(),
            dictionary: HashMap::new(),
        }
    }

    fn parse(&mut self, input: &str) {
        let mut tokens = input.split_whitespace().peekable();
        let mut defining: Option<String> = None;
        let mut buffer: Vec<Instruction> = Vec::new();

        while let Some(token) = tokens.next() {
            match token {
                ":" => {
                    let name = tokens.next().expect("Expected word name after ':'");
                    defining = Some(name.to_string());
                    buffer.clear();
                }
                ";" => {
                    if let Some(name) = defining.take() {
                        buffer.push(Instruction::Return);
                        let addr = self.main.len() + self.definitions.len() + 1; // +1 for future HALT
                        self.dictionary.insert(name, addr);
                        self.definitions.extend(buffer.drain(..));
                    } else {
                        panic!("Unexpected ';' outside of word definition");
                    }
                }
                word => {
                    let instr = if let Ok(n) = word.parse::<i32>() {
                        Instruction::Push(n)
                    } else {
                        match word {
                            "dup" => Instruction::Dup,
                            "drop" => Instruction::Drop,
                            "swap" => Instruction::Swap,
                            "over" => Instruction::Over,
                            "+" => Instruction::Add,
                            "*" => Instruction::Mul,
                            "depth" => Instruction::Depth,
                            _ => Instruction::CallWord(word.to_string()),
                        }
                    };

                    if defining.is_some() {
                        buffer.push(instr);
                    } else {
                        self.main.push(instr);
                    }
                }
            }
        }
    }

    fn finalize(self) -> (Vec<Instruction>, HashMap<String, usize>) {
        let mut instructions = self.main;
        instructions.push(Instruction::Halt); // ✅ main program ends here
        instructions.extend(self.definitions);
        (instructions, self.dictionary)
    }
}


fn main() {
    let mut parser = Parser::new();
    parser.parse("5 square : square dup * ;");

    let (instructions, dictionary) = parser.finalize();

    for instr in &instructions {
        println!("{:?}", instr);
    }

    let mut vm = VM::new(instructions);
    vm.dictionary = dictionary;
    vm.run();

    println!("Final stack: {:?}", vm.stack); // Should be [25]
}
