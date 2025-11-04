#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    ValBool(bool),
    ValNumber(f64),
    ValNil,
}

#[derive(Debug, Clone)]
pub enum OpCode {
    OpConstant(Value),
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
    OpNil,
    OpTrue,
    OpFalse,
    OpNot,
    OpEqual,
    OpGreater,
    OpLess,
    OpReturn,
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: vec![],
            lines: vec![],
        }
    }

    pub fn write(&mut self, op: OpCode, line: usize) {
        self.code.push(op);
        self.lines.push(line);
    }
}

#[derive(Debug)]
pub struct VirtualMachine {
    pub chunk: Chunk,
    pub ip: usize,
    pub stack: Vec<Value>,
}

impl VirtualMachine {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: vec![],
        }
    }

    pub fn step_once(&mut self) {
        if self.ip >= self.chunk.code.len() {
            return;
        }

        let op = self.chunk.code[self.ip].clone();
        self.ip += 1;

        match op {
            OpCode::OpConstant(v) => self.stack.push(v),
            OpCode::OpAdd => self.binary_op(|a, b| a + b),
            OpCode::OpSubtract => self.binary_op(|a, b| a - b),
            OpCode::OpMultiply => self.binary_op(|a, b| a * b),
            OpCode::OpDivide => self.binary_op(|a, b| a / b),
            OpCode::OpNegate => {
                if let Some(Value::ValNumber(a)) = self.stack.pop() {
                    self.stack.push(Value::ValNumber(-a));
                }
            }
            OpCode::OpReturn => {
                if let Some(v) = self.stack.last() {
                    println!("=> {:?}", v);
                }
            }
            _ => {}
        }
    }

    fn binary_op<F>(&mut self, op: F)
    where
        F: Fn(f64, f64) -> f64,
    {
        if let (Some(Value::ValNumber(b)), Some(Value::ValNumber(a))) =
            (self.stack.pop(), self.stack.pop())
        {
            self.stack.push(Value::ValNumber(op(a, b)));
        }
    }
}
