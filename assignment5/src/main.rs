mod virtual_machine;

use virtual_machine::*;

fn main() {
    println!("Running sample program...");

    let mut chunk = Chunk::new();
    chunk.write(OpCode::OpNil, 1);
    chunk.write(OpCode::OpReturn, 1);

    let mut vm = VirtualMachine::new(chunk);
    vm.run();
}
