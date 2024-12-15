/// **Advent of Code 2024 Day 3**
fn main() {
    let input = include_str!("../input");
    let mut program = parse_memory(input);

    let result = run_program(&program);

    program.retain(|op| matches!(op, Operation::Mul(_, _)));
    let result_mul_only = run_program(&program);

    println!("03.1: {result_mul_only}");
    println!("03.2: {result}");
}

/// Represents different types of memory operations.
enum Operation {
    /// Multiplies two integers.
    Mul(i32, i32),
    /// Enables subsequent operations.
    Do,
    /// Disables subsequent operations.
    Dont,
}

/// Parses a string containing memory operations and returns a vector of `Operation`s.
///
/// The input string is expected to contain one or more operations.
/// Each operation can be one of the following:
///
/// - **Multiplication:** `mul(a,b)`, where `a` and `b` are integers.
/// - **Do:** `do()`
/// - **Don't:** `don't()`
fn parse_memory(input: &str) -> Vec<Operation> {
    let re = regex::Regex::new(r"(?<op>mul\((?<a>\d+),(?<b>\d+)\)|do\(\)|don't\(\))").unwrap();

    re.captures_iter(input)
        .map(|cap| match &cap["op"] {
            op if op.starts_with("mul") => {
                Operation::Mul(cap["a"].parse().unwrap(), cap["b"].parse().unwrap())
            }
            "do()" => Operation::Do,
            "don't()" => Operation::Dont,
            _ => unreachable!(),
        })
        .collect()
}

/// **Both Stars**
///
/// Executes a program of `Operation`s and returns the final sum.
///
/// The program consists of a sequence of operations that can either multiply numbers or toggle the enabled state.
///
/// - **Multiplication:** `Operation::Mul(a, b)` multiplies `a` by `b` and adds the result to the sum if the program is enabled.
/// - **Enable:** `Operation::Do` enables the calculations, allowing subsequent multiplication operations to take effect.
/// - **Disable:** `Operation::Dont` disables the calculations, preventing subsequent multiplication operations from affecting the sum.
fn run_program(program: &[Operation]) -> i32 {
    let mut sum = 0;
    let mut enabled = true;

    for op in program {
        match op {
            Operation::Mul(a, b) if enabled => sum += a * b,
            Operation::Do => enabled = true,
            Operation::Dont => enabled = false,
            _ => (),
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_run_program_mul_only() {
        let mut program = parse_memory(TEST_DATA);
        program.retain(|op| matches!(op, Operation::Mul(_, _)));
        let result = run_program(&program);
        assert_eq!(161, result);
    }

    #[test]
    fn test_run_program_complete() {
        let program = parse_memory(TEST_DATA);
        let result = run_program(&program);
        assert_eq!(48, result);
    }
}
