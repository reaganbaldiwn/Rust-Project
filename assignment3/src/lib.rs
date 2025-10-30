pub mod scanner;
use scanner::*;

#[derive(Debug)]
pub enum InterpretResult {
    InterpretSuccess,
    InterpretCompileError,
    InterpretRuntimeError,
}

pub struct VirtualMachine;

impl VirtualMachine {
    pub fn init_machine() -> Self {
        VirtualMachine
    }

    // Interpret Lox source code
    pub fn interpret(&mut self, source_code: &str) -> InterpretResult {
        self.compile(source_code);
        InterpretResult::InterpretSuccess
    }

    // Compile by scanning and printing tokens
    pub fn compile(&mut self, source_code: &str) {
        let mut scanner: Scanner = Scanner::init_scanner(source_code);
        let mut line: usize = 0;

        loop {
            let token: Token = scanner.scan_token();

            if token.line != line {
                print!("{:4} ", token.line);
                line = token.line;
            } else {
                print!("   | ");
            }

            let text = String::from_utf8_lossy(&token.value);
            println!("{:?} {}, {:?}", token.token_type, token.value.len(), text);

            if let TokenType::TokenEof = token.token_type {
                break;
            }
        }
    }
}
