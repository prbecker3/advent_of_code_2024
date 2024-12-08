use std::collections::HashMap;

/// **Advent of Code 2024 Day 1**
fn main() {
    let input = include_str!("input");
    let (list1, list2) = build_lists(input);

    let total_distance = calculate_total_distance(&list1, &list2);
    let similarity_score = calculate_similarity_score(&list1, &list2);

    println!("01.1: {total_distance}");
    println!("01.2: {similarity_score}");
}

/// Parses the input string into two sorted vectors of integers.
/// The input string is expected to contain lines with exactly two space-separated integers.
fn build_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (mut list1, mut list2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut iter = line
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok());
            (iter.next().unwrap(), iter.next().unwrap())
        })
        .unzip();

    list1.sort();
    list2.sort();

    (list1, list2)
}

/// **Star 1**
///
/// Calculates the total absolute difference between corresponding elements of two integer lists.
fn calculate_total_distance(list1: &[i32], list2: &[i32]) -> i32 {
    list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

/// **Star 2**
///
/// Calculates a similarity score between two integer lists.
///
/// This function calculates a similarity score based on the number of matching elements
/// between the two lists. It counts the occurrences of each element in the second list
/// and multiplies each element in the first list by its corresponding count in the second list.
/// The sum of these products is the similarity score.
fn calculate_similarity_score(list1: &[i32], list2: &[i32]) -> i32 {
    let counts = list2.iter().fold(HashMap::new(), |mut counts, num| {
        *counts.entry(*num).or_insert(0) += 1;
        counts
    });

    list1
        .iter()
        .map(|num| num * counts.get(num).copied().unwrap_or_default())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_calculate_total_distance() {
        let (list1, list2) = build_lists(TEST_DATA);
        let result = calculate_total_distance(&list1, &list2);
        assert_eq!(11, result);
    }

    #[test]
    fn test_calculate_similarity_score() {
        let (list1, list2) = build_lists(TEST_DATA);
        let result = calculate_similarity_score(&list1, &list2);
        assert_eq!(31, result)
    }
}
