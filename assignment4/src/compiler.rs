use crate::vm::{Chunk, OpCode};

pub struct Compiler {
    source: String,
}

impl Compiler {
    pub fn new(source: String) -> Self {
        Self { source }
    }

    pub fn compile(&mut self) -> Chunk {
        let mut chunk = Chunk::new();

        // Very simple expression parser for demo (supports + - * /)
        let tokens: Vec<&str> = self.source.split_whitespace().collect();

        if tokens.len() == 1 {
            let val: f64 = tokens[0].parse().unwrap();
            chunk.write(OpCode::OpConstant(val));
        } else if tokens.len() == 3 {
            let a: f64 = tokens[0].parse().unwrap();
            let op = tokens[1];
            let b: f64 = tokens[2].parse().unwrap();

            chunk.write(OpCode::OpConstant(a));
            chunk.write(OpCode::OpConstant(b));

            match op {
                "+" => chunk.write(OpCode::OpAdd),
                "-" => chunk.write(OpCode::OpSubtract),
                "*" => chunk.write(OpCode::OpMultiply),
                "/" => chunk.write(OpCode::OpDivide),
                _ => {}
            }
        }

        chunk.write(OpCode::OpReturn);
        chunk
    }
}
