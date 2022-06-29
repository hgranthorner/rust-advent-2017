use std::fs;
use std::path::Path;
use std::str::FromStr;

pub fn solve(path: &Path) -> std::io::Result<(u32, u32)> {
    let contents = fs::read_to_string(path)?;
    let solution1 = checksum(&contents, determine_difference);
    let solution2 = checksum(&contents, determine_divisible_result);
    Ok((solution1, solution2))
}

fn checksum(rows: &str, f: fn(&str) -> u32) -> u32 {
    rows.lines().into_iter().map(f).sum()
}

fn determine_difference(row: &str) -> u32 {
    if row.is_empty() {
        return 0;
    }

    let nums: Vec<_> = row
        .split_whitespace()
        .into_iter()
        .map(|n| u32::from_str(n).unwrap())
        .collect();

    let (mn, mx) = (*nums.iter().min().unwrap(), *nums.iter().max().unwrap());
    mx - mn
}

fn determine_divisible_result(row: &str) -> u32 {
    if row.is_empty() {
        return 0;
    }

    let nums: Vec<_> = row
        .split_whitespace()
        .into_iter()
        .map(|n| u32::from_str(n).unwrap())
        .collect();

    let mut result = 0;
    for i in 0..nums.len() {
        let current_num = nums[i];
        for (j, comparison_num) in nums.iter().enumerate() {
            if i == j {
                continue;
            }

            if current_num % comparison_num == 0 {
                result = (current_num / comparison_num).max(comparison_num / current_num);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_difference() {
        assert_eq!(determine_difference("0 1 2 3"), 3);
        assert_eq!(determine_difference("1\t32\t0"), 32);
    }

    #[test]
    fn test_determine_divisible_result() {
        assert_eq!(determine_divisible_result("2 3 4 5"), 2);
        assert_eq!(determine_divisible_result("32\t16\t3"), 2);
    }

    #[test]
    fn test_checksum() {
        assert_eq!(
            checksum(
                "5 1 9 5
7 5 3
2 4 6 8",
                determine_difference
            ),
            18
        );
        assert_eq!(
            checksum(
                "5 9 2 8
9 4 7 3
3 8 6 5",
                determine_divisible_result
            ),
            9
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(
            solve(Path::new("./inputs/day_two.txt")).unwrap(),
            (50376, 267)
        );
    }
}
