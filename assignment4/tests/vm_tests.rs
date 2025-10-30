use assignment4::vm::{VirtualMachine, InterpretResult};

fn run_expression(expr: &str) -> f64 {
    let mut vm = VirtualMachine::new();
    match vm.interpret(expr) {
        InterpretResult::Ok(val) => val,
        _ => panic!("Runtime error in expression: {}", expr),
    }
}

#[test]
fn test_single_number() {
    assert_eq!(run_expression("7"), 7.0);
}

#[test]
fn test_addition() {
    assert_eq!(run_expression("1 + 8"), 9.0);
    assert_eq!(run_expression("3 + 4"), 7.0);
    assert_eq!(run_expression("10 + 2"), 12.0);
}

#[test]
fn test_multiplication() {
    assert_eq!(run_expression("3 * 4"), 12.0);
    assert_eq!(run_expression("2 * 5"), 10.0);
    assert_eq!(run_expression("6 * 7"), 42.0);
}

#[test]
fn test_subtraction() {
    assert_eq!(run_expression("5 - 3"), 2.0);
    assert_eq!(run_expression("9 - 4"), 5.0);
    assert_eq!(run_expression("10 - 2"), 8.0);
}

#[test]
fn test_division() {
    assert_eq!(run_expression("6 / 2"), 3.0);
    assert_eq!(run_expression("9 / 3"), 3.0);
    assert_eq!(run_expression("12 / 4"), 3.0);
}
