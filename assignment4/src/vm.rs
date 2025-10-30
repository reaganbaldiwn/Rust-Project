#[derive(Debug, Clone)]
pub enum OpCode {
    OpConstant(f64),
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpReturn,
}

pub struct Chunk {
    pub code: Vec<OpCode>,
}

impl Chunk {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }

    pub fn write(&mut self, opcode: OpCode) {
        self.code.push(opcode);
    }
}

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Ok(f64),
    RuntimeError,
}

use crate::compiler::Compiler;

pub struct VirtualMachine {
    chunk: Chunk,
    stack: Vec<f64>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, source_code: &str) -> InterpretResult {
        let mut compiler = Compiler::new(source_code.to_string());
        self.chunk = compiler.compile();

        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        for op in &self.chunk.code {
            match op {
                OpCode::OpConstant(value) => self.stack.push(*value),
                OpCode::OpAdd => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }
                OpCode::OpSubtract => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a - b);
                }
                OpCode::OpMultiply => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                }
                OpCode::OpDivide => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a / b);
                }
                OpCode::OpReturn => {
                    return InterpretResult::Ok(self.stack.pop().unwrap());
                }
            }
        }

        InterpretResult::RuntimeError
    }
}
