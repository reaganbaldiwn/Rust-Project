use assignment1::{Chunk, OpCode};

fn main() {
    let mut chunk = Chunk::init_chunk();

    // Add a constant
    chunk.add_constant(42, 1);

    // Add arithmetic operations
    chunk.write_to_chunk(OpCode::OpNegate.to_byte(), 2);
    chunk.write_to_chunk(OpCode::OpAdd.to_byte(), 3);
    chunk.write_to_chunk(OpCode::OpSubtract.to_byte(), 4);
    chunk.write_to_chunk(OpCode::OpMultiply.to_byte(), 5);
    chunk.write_to_chunk(OpCode::OpDivide.to_byte(), 6);

    // Add return
    chunk.write_to_chunk(OpCode::OpReturn.to_byte(), 7);

    // Disassemble
    chunk.disassemble("Test Chunk");
}
