use assignment2::*;

fn main() {
    let mut chunk = Chunk::init_chunk();
    chunk.add_constant(10);
    chunk.add_constant(2);

    chunk.write_to_chunk(OpCode::OpAdd.to_u8(), 1);
    chunk.write_to_chunk(OpCode::OpMultiply.to_u8(), 2);
    chunk.write_to_chunk(OpCode::OpDivide.to_u8(), 3);
    chunk.write_to_chunk(OpCode::OpReturn.to_u8(), 4);

    chunk.disassemble("Test Chunk");

    let mut vm = VirtualMachine::init_machine();
    let result = vm.interpret(chunk);
    println!("Interpretation result: {:?}", result);
}
