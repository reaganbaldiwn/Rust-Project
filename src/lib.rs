// Define a Value as an alias for u8
pub type Value = u8;

// Operation codes supported by our VM
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    OpReturn,
    OpConstant,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
}

impl OpCode {
    // Convert an OpCode to a byte
    pub fn to_byte(self) -> u8 {
        match self {
            OpCode::OpReturn   => 0,
            OpCode::OpConstant => 1,
            OpCode::OpNegate   => 2,
            OpCode::OpAdd      => 3,
            OpCode::OpSubtract => 4,
            OpCode::OpMultiply => 5,
            OpCode::OpDivide   => 6,
        }
    }

    // Convert a byte back into an OpCode
    pub fn from_byte(byte: u8) -> Option<OpCode> {
        match byte {
            0 => Some(OpCode::OpReturn),
            1 => Some(OpCode::OpConstant),
            2 => Some(OpCode::OpNegate),
            3 => Some(OpCode::OpAdd),
            4 => Some(OpCode::OpSubtract),
            5 => Some(OpCode::OpMultiply),
            6 => Some(OpCode::OpDivide),
            _ => None,
        }
    }
}

// Represents a chunk of bytecode + metadata
pub struct Chunk {
    pub code: Vec<u8>,      // bytecode instructions + operands
    pub lines: Vec<u32>,    // line numbers from source code
    pub values: Vec<Value>, // constants table
}

impl Chunk {
    // Initialize an empty chunk
    pub fn init_chunk() -> Self {
        Chunk {
            code: Vec::new(),
            lines: Vec::new(),
            values: Vec::new(),
        }
    }

    // Write a byte to the code vector and record the line number
    pub fn write_to_chunk(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        self.lines.push(line);
    }

    // Add a constant to the chunk and write OpConstant + index
    pub fn add_constant(&mut self, value: Value, line: u32) {
        self.values.push(value);
        let index = (self.values.len() - 1) as u8;
        self.write_to_chunk(OpCode::OpConstant.to_byte(), line);
        self.write_to_chunk(index, line);
    }

    // Disassemble the entire chunk with a header name
    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    // Disassemble a single instruction at offset
    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        let byte = self.code[offset];
        match OpCode::from_byte(byte) {
            Some(op) => match op {
                OpCode::OpReturn
                | OpCode::OpNegate
                | OpCode::OpAdd
                | OpCode::OpSubtract
                | OpCode::OpMultiply
                | OpCode::OpDivide => {
                    println!("{:?}", op);
                    offset + 1
                }
                OpCode::OpConstant => {
                    let const_index = self.code[offset + 1] as usize;
                    let value = self.values[const_index];
                    println!("{:?} {} (value={})", op, const_index, value);
                    offset + 2
                }
            },
            None => {
                println!("Unknown opcode {}", byte);
                offset + 1
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_conversion() {
        let ops = [
            OpCode::OpReturn,
            OpCode::OpConstant,
            OpCode::OpNegate,
            OpCode::OpAdd,
            OpCode::OpSubtract,
            OpCode::OpMultiply,
            OpCode::OpDivide,
        ];

        for op in ops {
            let byte = op.to_byte();
            assert_eq!(OpCode::from_byte(byte), Some(op));
        }

        assert_eq!(OpCode::from_byte(99), None);
    }

    #[test]
    fn test_chunk_add_constant() {
        let mut chunk = Chunk::init_chunk();
        chunk.add_constant(7, 1);

        assert_eq!(chunk.values.len(), 1);
        assert_eq!(chunk.values[0], 7);
        assert_eq!(chunk.code[0], OpCode::OpConstant.to_byte());
        assert_eq!(chunk.code[1], 0);
        assert_eq!(chunk.lines[0], 1);
    }
}
