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
    Return,
    Halt,
}

#[derive(Debug)]
struct VM {
    stack: Vec<i32>,
    program: Vec<Instruction>,
    ip: usize,
    return_stack: Vec<usize>,
}

impl VM {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            stack: Vec::new(),
            program,
            ip: 0,
            return_stack: Vec::new(),
        }
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

fn main() {
    let program = vec![
        // main
        Instruction::Push(5),       // [5]
        Instruction::Call(3),       // jump to square
        Instruction::Halt,

        // square (addr 5)
        Instruction::Dup,           // [5, 5]
        Instruction::Mul,           // [25]
        Instruction::Return,
    ];

    let mut vm = VM::new(program);
    vm.run();

    println!("Final stack: {:?}", vm.stack); // Should be [20]
}
