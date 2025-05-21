#[derive(Debug)]
enum Instruction {
    Push(i32),
    Add,
    Mul,
    Dup,
    Drop,
    Swap,
    Halt,
}

#[derive(Debug)]
struct VM {
    stack: Vec<i32>,
    program: Vec<Instruction>,
    ip: usize, // instruction pointer
}

impl VM {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            stack: Vec::new(),
            program,
            ip: 0,
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
                Instruction::Halt => break,
            }
            self.ip += 1;
        }
    }
}

fn main() {
    let program = vec![
        Instruction::Push(2),
        Instruction::Push(3),
        Instruction::Add,
        Instruction::Push(4),
        Instruction::Mul,
        Instruction::Halt,
    ];

    let mut vm = VM::new(program);
    vm.run();

    println!("Final stack: {:?}", vm.stack); // Should be [20]
}
