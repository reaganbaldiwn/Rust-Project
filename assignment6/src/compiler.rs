use crate::virtual_machine::{Chunk, Op, Value, VirtualMachine};

#[derive(Clone)]
pub struct ParseRule {
    // Placeholder for potential parsing rules
}

impl ParseRule {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct Compiler;

impl Compiler {
    pub fn new() -> Self {
        Self
    }

    pub fn compile(&self, source: &str) -> Chunk {
        let mut chunk = Chunk::new();

        // If the source looks like a number, add it as a constant
        if let Ok(num) = source.trim().parse::<f64>() {
            let index = chunk.add_constant(Value::ValNumber(num));
            if index >= 0 {
                chunk.write(Op::OpConstant);
            }
        }

        // Example: append an OpReturn so the VM knows to stop
        chunk.write(Op::OpReturn);
        chunk
    }
}

pub fn run_source(source: &str) {
    let compiler = Compiler::new();
    let chunk = compiler.compile(source);
    let mut vm = VirtualMachine::new(chunk);
    vm.interpret();
}
