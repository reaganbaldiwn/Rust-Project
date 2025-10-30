use assignment4::vm::VirtualMachine;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: assignment4 <file>");
        return;
    }

    let source = fs::read_to_string(&args[1]).expect("Failed to read file");
    let mut vm = VirtualMachine::new();
    let result = vm.interpret(&source);

    println!("Result: {:?}", result);
}
