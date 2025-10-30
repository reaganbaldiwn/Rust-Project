use std::env;
use std::fs;
use assignment3::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <lox_source_file>");
        return;
    }

    let file_path = &args[1];
    let source = fs::read_to_string(file_path).expect("Failed to read source file");

    let mut vm = VirtualMachine::init_machine();
    vm.interpret(&source);
}
