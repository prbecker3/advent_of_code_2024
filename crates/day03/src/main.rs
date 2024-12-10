use regex::Regex;
use std::ops::Range;

/// **Advent of Code 2024 Day 3**
fn main() {
    let input = include_str!("input");
    let mulops = parse_mulops(input);
    let enabled_ranges = parse_enabled_ranges(input);

    let sum_of_all_products = sum_of_all_products(&mulops);
    let sum_of_enabled_products = sum_of_enabled_products(&mulops, &enabled_ranges);

    println!("03.1: {sum_of_all_products}");
    println!("03.2: {sum_of_enabled_products}");
}

/// Parses a string containing multiplication operations into a vector of `MulOp` structs.
///
/// The input string is expected to contain one or more multiplication operations in the format
/// `mul(a,b)`, where `a` and `b` are integer values. The function parses each operation and
/// creates a corresponding `MulOp` struct, storing the factors and the starting index of the
/// operation in the input string.
fn parse_mulops(memory: &str) -> Vec<MulOp> {
    let re = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();

    re.captures_iter(memory)
        .map(|capture| MulOp {
            factor1: capture["a"].parse().unwrap(),
            factor2: capture["b"].parse().unwrap(),
            index: capture.get(0).unwrap().start(),
        })
        .collect()
}

/// Parses a string containing "do()" and "don't()" markers to identify enabled ranges.
///
/// This function takes a string as input and identifies the ranges of characters that are considered "enabled."
/// An enabled range starts at the index of a "do()" marker and ends at the index of the next "don't()" marker.
/// If there are no more "don't()" markers after a "do()" marker, the range extends to the end of the string.
fn parse_enabled_ranges(memory: &str) -> Vec<Range<usize>> {
    let mut do_indices: Vec<usize> = memory.match_indices("do()").map(|index| index.0).collect();
    do_indices.insert(0, 0);

    let mut enabled_ranges: Vec<Range<usize>> = memory
        .match_indices("don't()")
        .filter_map(|index| {
            if let Some(&do_index) = do_indices.first() {
                if index.0 > do_index {
                    do_indices.retain(|i| i > &index.0);
                    return Some(do_index..index.0);
                }
            }

            None
        })
        .collect();

    if let Some(&do_index) = do_indices.first() {
        enabled_ranges.push(do_index..memory.len());
    }

    enabled_ranges
}

/// **Star 1**
///
/// Calculates the sum of the products of all `MulOp` instances in the given slice.
fn sum_of_all_products(mulops: &[MulOp]) -> i32 {
    mulops.iter().map(MulOp::product).sum()
}

/// **Star 2**
///
/// Calculates the sum of products of `MulOp` instances within enabled ranges.
fn sum_of_enabled_products(mulops: &[MulOp], enabled_ranges: &[Range<usize>]) -> i32 {
    mulops
        .iter()
        .filter(|op| enabled_ranges.iter().any(|range| range.contains(&op.index)))
        .map(MulOp::product)
        .sum()
}

/// Represents a multiplication operation with two factors and an index.
struct MulOp {
    factor1: i32,
    factor2: i32,
    index: usize,
}

impl MulOp {
    /// Calculates the product of the two factors.
    fn product(&self) -> i32 {
        self.factor1 * self.factor2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_sum_of_all_products() {
        let mulops = parse_mulops(TEST_DATA);
        let result = sum_of_all_products(&mulops);
        assert_eq!(161, result);
    }

    #[test]
    fn test_sum_of_enabled_products() {
        let mulops = parse_mulops(TEST_DATA);
        let enabled_ranges = parse_enabled_ranges(TEST_DATA);
        let result = sum_of_enabled_products(&mulops, &enabled_ranges);
        assert_eq!(48, result);
    }
}
