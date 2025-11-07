mod virtual_machine;
mod compiler;

use compiler::run_source;

fn main() {
    // Example: print the number 3.14
    run_source("3.14");
}

