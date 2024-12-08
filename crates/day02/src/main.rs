/// **Advent of Code 2024 Day 2**
fn main() {
    let input = include_str!("input");
    let reports = load_reports(input);

    let safe_reports = count_safe_reports(&reports);
    let safe_reports_with_dampener = count_safe_reports_with_dampener(&reports);

    println!("02.1: {safe_reports}");
    println!("02.2: {safe_reports_with_dampener}");
}

/// Parses a multi-line string into a vector of integer vectors.
///
/// Each line in the input string is expected to contain a sequence of whitespace-separated integers.
fn load_reports(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect()
        })
        .collect()
}

/// Checks if a report is considered "safe" based on step size limitations.
///
/// A report is considered safe if:
/// * All consecutive step sizes (differences between adjacent elements) are less than or equal to 3 (in absolute value).
/// * The report is either strictly increasing (all steps positive) or strictly decreasing (all steps negative).
fn report_is_safe(report: &[i32]) -> bool {
    let is_incremental = report[0] < report[1];

    report.windows(2).all(|pair| {
        let step = pair[1] - pair[0];
        step.abs() <= 3 && (is_incremental && step > 0 || !is_incremental && step < 0)
    })
}

/// **Star 1**
///
/// Counts the number of safe reports in a collection.
fn count_safe_reports(reports: &[Vec<i32>]) -> usize {
    reports
        .iter()
        .filter(|report| report_is_safe(report))
        .count()
}

/// **Star 2**
///
/// Counts the number of reports that become safe after removing a single element.
///
/// This function builds upon the concept of safe reports defined in `report_is_safe`. Here, it iterates through each report and checks if removing any single element from the report would result in a safe report.
fn count_safe_reports_with_dampener(reports: &[Vec<i32>]) -> usize {
    reports
        .iter()
        .filter(|report| {
            if report_is_safe(report) {
                return true;
            }

            (0..report.len()).any(|i| {
                let mut dampened_report = (*report).clone();
                dampened_report.remove(i);
                report_is_safe(&dampened_report)
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_count_safe_reports() {
        let reports = load_reports(TEST_DATA);
        let result = count_safe_reports(&reports);
        assert_eq!(2, result);
    }

    #[test]
    fn test_count_safe_reports_with_dampener() {
        let reports = load_reports(TEST_DATA);
        let result = count_safe_reports_with_dampener(&reports);
        assert_eq!(4, result);
    }
}
