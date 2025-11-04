use std::fmt;

// Number alias
pub type Number = f64;

// === Value Enum ===
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    ValBool(bool),
    ValNumber(Number),
    ValNil,
}

// === OpCode Enum ===
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

// === Chunk Structure ===
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, op: OpCode, line: usize) {
        self.code.push(op);
        self.lines.push(line);
    }
}

// === VirtualMachine ===
pub struct VirtualMachine {
    pub chunk: Chunk,
    pub ip: usize,
    pub stack: Vec<Value>,
}

impl VirtualMachine {
    pub fn new(chunk: Chunk) -> Self {
        VirtualMachine {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    // === Run method ===
    pub fn run(&mut self) -> Option<Value> {
        while self.ip < self.chunk.code.len() {
            let instruction = self.chunk.code[self.ip].clone();
            self.ip += 1;

            match instruction {
                OpCode::OpConstant(val) => self.stack.push(val),
                OpCode::OpAdd => self.binary_op(|a, b| a + b),
                OpCode::OpSubtract => self.binary_op(|a, b| a - b),
                OpCode::OpMultiply => self.binary_op(|a, b| a * b),
                OpCode::OpDivide => self.binary_op(|a, b| a / b),
                OpCode::OpNegate => {
                    if let Some(Value::ValNumber(n)) = self.stack.pop() {
                        self.stack.push(Value::ValNumber(-n));
                    } else {
                        self.runtime_error("Operand must be a number.");
                        return None;
                    }
                }
                OpCode::OpNil => self.stack.push(Value::ValNil),
                OpCode::OpTrue => self.stack.push(Value::ValBool(true)),
                OpCode::OpFalse => self.stack.push(Value::ValBool(false)),
                OpCode::OpNot => {
                    if let Some(val) = self.stack.pop() {
                        self.stack.push(Value::ValBool(self.is_falsey(val)));
                    } else {
                        self.runtime_error("Stack underflow on NOT.");
                        return None;
                    }
                }
                OpCode::OpEqual => self.binary_cmp(|a, b| a == b),
                OpCode::OpGreater => self.binary_cmp(|a, b| a > b),
                OpCode::OpLess => self.binary_cmp(|a, b| a < b),
                OpCode::OpReturn => {
                    if let Some(val) = self.stack.last() {
                        println!("=> {:?}", val);
                        return Some(*val);
                    } else {
                        println!("=> (empty stack)");
                        return None;
                    }
                }
            }
        }
        None
    }

    // === Helper functions ===
    fn binary_op<F>(&mut self, op: F)
    where
        F: Fn(f64, f64) -> f64,
    {
        if let (Some(Value::ValNumber(b)), Some(Value::ValNumber(a))) =
            (self.stack.pop(), self.stack.pop())
        {
            self.stack.push(Value::ValNumber(op(a, b)));
        } else {
            self.runtime_error("Operands must be numbers.");
        }
    }

    fn binary_cmp<F>(&mut self, cmp: F)
    where
        F: Fn(f64, f64) -> bool,
    {
        if let (Some(Value::ValNumber(b)), Some(Value::ValNumber(a))) =
            (self.stack.pop(), self.stack.pop())
        {
            self.stack.push(Value::ValBool(cmp(a, b)));
        } else {
            self.runtime_error("Operands must be numbers.");
        }
    }

    fn is_falsey(&self, val: Value) -> bool {
        match val {
            Value::ValBool(false) | Value::ValNil => true,
            _ => false,
        }
    }

    fn runtime_error(&self, message: &str) {
        println!("{}", message);
        if self.ip < self.chunk.lines.len() {
            println!("[line {}] in script", self.chunk.lines[self.ip]);
        }
    }
}

// === Unit Tests ===
#[cfg(test)]
mod tests {
    use super::*;

    fn make_vm_with_ops(ops: Vec<OpCode>) -> VirtualMachine {
        let mut chunk = Chunk::new();
        for op in ops {
            chunk.write(op, 1);
        }
        VirtualMachine::new(chunk)
    }

    #[test]
    fn test_addition() {
        let ops = vec![
            OpCode::OpConstant(Value::ValNumber(3.0)),
            OpCode::OpConstant(Value::ValNumber(4.0)),
            OpCode::OpAdd,
            OpCode::OpReturn,
        ];
        let mut vm = make_vm_with_ops(ops);
        assert_eq!(vm.run(), Some(Value::ValNumber(7.0)));
    }

    #[test]
    fn test_subtraction() {
        let ops = vec![
            OpCode::OpConstant(Value::ValNumber(10.0)),
            OpCode::OpConstant(Value::ValNumber(3.0)),
            OpCode::OpSubtract,
            OpCode::OpReturn,
        ];
        let mut vm = make_vm_with_ops(ops);
        assert_eq!(vm.run(), Some(Value::ValNumber(7.0)));
    }

    #[test]
    fn test_multiplication() {
        let ops = vec![
            OpCode::OpConstant(Value::ValNumber(6.0)),
            OpCode::OpConstant(Value::ValNumber(7.0)),
            OpCode::OpMultiply,
            OpCode::OpReturn,
        ];
        let mut vm = make_vm_with_ops(ops);
        assert_eq!(vm.run(), Some(Value::ValNumber(42.0)));
    }

    #[test]
    fn test_division() {
        let ops = vec![
            OpCode::OpConstant(Value::ValNumber(8.0)),
            OpCode::OpConstant(Value::ValNumber(2.0)),
            OpCode::OpDivide,
            OpCode::OpReturn,
        ];
        let mut vm = make_vm_with_ops(ops);
        assert_eq!(vm.run(), Some(Value::ValNumber(4.0)));
    }

    #[test]
    fn test_negation() {
        let ops = vec![
            OpCode::OpConstant(Value::ValNumber(3.0)),
            OpCode::OpNegate,
            OpCode::OpReturn,
        ];
        let mut vm = make_vm_with_ops(ops);
        assert_eq!(vm.run(), Some(Value::ValNumber(-3.0)));
    }

    #[test]
    fn test_not_operator() {
        let ops = vec![
            OpCode::OpFalse,
            OpCode::OpNot,
            OpCode::OpReturn,
        ];
        let mut vm = make_vm_with_ops(ops);
        assert_eq!(vm.run(), Some(Value::ValBool(true)));
    }

    #[test]
    fn test_comparisons() {
        let ops = vec![
            OpCode::OpConstant(Value::ValNumber(5.0)),
            OpCode::OpConstant(Value::ValNumber(3.0)),
            OpCode::OpGreater,
            OpCode::OpReturn,
        ];
        let mut vm = make_vm_with_ops(ops);
        assert_eq!(vm.run(), Some(Value::ValBool(true)));
    }

    #[test]
    fn test_nil_and_boolean_push() {
        let ops = vec![
            OpCode::OpNil,
            OpCode::OpTrue,
            OpCode::OpFalse,
            OpCode::OpReturn,
        ];
        let mut vm = make_vm_with_ops(ops);
        assert_eq!(vm.run(), Some(Value::ValBool(false)));
    }
}
