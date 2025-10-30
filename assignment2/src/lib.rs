// ---------- Value Type ----------
pub type Value = u8;

// ---------- OpCode Enum ----------
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    OpReturn,
    OpConstant,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpModulo,
}

impl OpCode {
    pub fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(OpCode::OpReturn),
            1 => Some(OpCode::OpConstant),
            2 => Some(OpCode::OpNegate),
            3 => Some(OpCode::OpAdd),
            4 => Some(OpCode::OpSubtract),
            5 => Some(OpCode::OpMultiply),
            6 => Some(OpCode::OpDivide),
            7 => Some(OpCode::OpModulo),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            OpCode::OpReturn => 0,
            OpCode::OpConstant => 1,
            OpCode::OpNegate => 2,
            OpCode::OpAdd => 3,
            OpCode::OpSubtract => 4,
            OpCode::OpMultiply => 5,
            OpCode::OpDivide => 6,
            OpCode::OpModulo => 7,
        }
    }
}

// ---------- Chunk Structure ----------
#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub lines: Vec<usize>,
    pub values: Vec<Value>,
}

impl Chunk {
    pub fn init_chunk() -> Self {
        Chunk {
            code: vec![],
            lines: vec![],
            values: vec![],
        }
    }

    pub fn write_to_chunk(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.values.push(value);
        let index = self.values.len() - 1;
        self.write_to_chunk(OpCode::OpConstant.to_u8(), 0);
        self.write_to_chunk(index as u8, 0);
        index
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        let instruction = self.code[offset];
        if let Some(op) = OpCode::from_u8(instruction) {
            match op {
                OpCode::OpReturn => {
                    println!("OpReturn");
                    offset + 1
                }
                OpCode::OpConstant => {
                    let constant_index = self.code[offset + 1] as usize;
                    let constant_value = self.values[constant_index];
                    println!("OpConstant {} (value={})", constant_index, constant_value);
                    offset + 2
                }
                _ => {
                    println!("{:?}", op);
                    offset + 1
                }
            }
        } else {
            println!("Unknown opcode {}", instruction);
            offset + 1
        }
    }
}

// ---------- Virtual Machine ----------
#[derive(Debug)]
pub struct VirtualMachine {
    pub chunk: Option<Chunk>,
    pub ip: usize,          // instruction pointer
    pub stack: Vec<Value>,  // value stack
}

// Result type of interpretation
#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    InterpretSuccess,
    InterpretCompileError,
    InterpretRuntimeError,
}

impl VirtualMachine {
    pub fn init_machine() -> Self {
        VirtualMachine {
            chunk: None,
            ip: 0,
            stack: vec![],
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = Some(chunk);
        self.ip = 0;
        self.run()
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = {
                let chunk = self.chunk.as_ref().unwrap();
                if self.ip >= chunk.code.len() {
                    return InterpretResult::InterpretRuntimeError;
                }
                chunk.code[self.ip]
            };

            self.ip += 1;

            let opcode = OpCode::from_u8(instruction);
            match opcode {
                Some(OpCode::OpReturn) => return InterpretResult::InterpretSuccess,

                Some(OpCode::OpConstant) => {
                    let constant_index = self.read_byte();
                    let value = self.chunk.as_ref().unwrap().values[constant_index as usize];
                    self.stack.push(value);
                }

                Some(OpCode::OpNegate) => {
                    if let Some(value) = self.stack.pop() {
                        self.stack.push((-((value as i8)) as u8) as u8);
                    } else {
                        return InterpretResult::InterpretRuntimeError;
                    }
                }

                Some(OpCode::OpAdd)
                | Some(OpCode::OpSubtract)
                | Some(OpCode::OpMultiply)
                | Some(OpCode::OpDivide)
                | Some(OpCode::OpModulo) => {
                    if self.stack.len() < 2 {
                        return InterpretResult::InterpretRuntimeError;
                    }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    let result = match opcode.unwrap() {
                        OpCode::OpAdd => a.wrapping_add(b),
                        OpCode::OpSubtract => a.wrapping_sub(b),
                        OpCode::OpMultiply => a.wrapping_mul(b),
                        OpCode::OpDivide => {
                            if b == 0 {
                                return InterpretResult::InterpretRuntimeError;
                            }
                            a.wrapping_div(b)
                        }
                        OpCode::OpModulo => {
                            if b == 0 {
                                return InterpretResult::InterpretRuntimeError;
                            }
                            a % b
                        }
                        _ => 0,
                    };
                    self.stack.push(result);
                }

                None => return InterpretResult::InterpretRuntimeError,
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        let chunk = self.chunk.as_ref().unwrap();
        let byte = chunk.code[self.ip];
        self.ip += 1;
        byte
    }
}

// ---------- Tests ----------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_addition() {
        let mut chunk = Chunk::init_chunk();
        chunk.add_constant(5);
        chunk.add_constant(7);
        chunk.write_to_chunk(OpCode::OpAdd.to_u8(), 0);
        chunk.write_to_chunk(OpCode::OpReturn.to_u8(), 0);

        let mut vm = VirtualMachine::init_machine();
        let result = vm.interpret(chunk);

        assert_eq!(result, InterpretResult::InterpretSuccess);
        assert_eq!(vm.stack.pop().unwrap(), 12);
    }
}
