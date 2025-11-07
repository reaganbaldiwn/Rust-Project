#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    ValBool(bool),
    ValNumber(f64),
    ValNil,
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    OpConstant,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
    OpReturn,
    OpModulo,
    OpPrint,
    OpDefineGlobal,
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<Op>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write(&mut self, op: Op) {
        self.code.push(op);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
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
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
        while self.ip < self.chunk.code.len() {
            let op = self.chunk.code[self.ip];
            self.ip += 1;

            match op {
                Op::OpConstant => {
                    if let Some(constant) = self.chunk.constants.get(0) {
                        self.stack.push(*constant);
                    }
                }
                Op::OpAdd => self.binary_op(|a, b| a + b),
                Op::OpSubtract => self.binary_op(|a, b| a - b),
                Op::OpMultiply => self.binary_op(|a, b| a * b),
                Op::OpDivide => self.binary_op(|a, b| a / b),
                Op::OpModulo => self.binary_op(|a, b| a % b),
                Op::OpNegate => {
                    if let Some(Value::ValNumber(v)) = self.stack.pop() {
                        self.stack.push(Value::ValNumber(-v));
                    }
                }
                Op::OpPrint => {
                    if let Some(value) = self.stack.last() {
                        println!("{:?}", value);
                    }
                }
                Op::OpReturn => {
                    if let Some(v) = self.stack.last() {
                        match v {
                            Value::ValNumber(num) => println!("{}", num),
                            Value::ValBool(b) => println!("{}", b),
                            Value::ValNil => println!("nil"),
                        }
                    }
                    return InterpretResult::InterpretOk;
                }
                Op::OpDefineGlobal => {} // placeholder for future variable definition
            }
        }

        InterpretResult::InterpretOk
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
