use std::fs;
use std::path::Path;

pub fn solve(path: &Path) -> std::io::Result<(u32, u32)> {
    let contents = fs::read_to_string(path)?;
    let solution = solve_captcha(&contents);
    let solution_2 = solve_captcha_2(&contents);
    Ok((solution, solution_2))
}

fn solve_captcha(digit_str: &str) -> u32 {
    let digits = digit_str
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let mut sum = 0;
    for n in 0..(digits.len() - 1) {
        let current_num = digits[n];
        if current_num == digits[n + 1] {
            sum += current_num;
        }
    }

    if digits.last().unwrap() == digits.first().unwrap() {
        sum += digits.last().unwrap();
    }

    sum
}

fn solve_captcha_2(digit_str: &str) -> u32 {
    let digits = digit_str
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let half_len = digits.len() / 2;
    let mut sum = 0;

    for n in 0..digits.len() {
        let current_num = digits[n];
        let mut next_index = n + half_len;
        println!("Looping!");
        println!("Indexes: {} - {}", n, next_index);
        if next_index >= digits.len() {
            next_index -= digits.len();
        }

        println!("Final indexes: {} - {}", n, next_index);
        println!("Final values: {} - {}", current_num, digits[next_index]);
        if current_num == digits[next_index] {
            sum += current_num;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_captcha() {
        assert_eq!(solve_captcha("1122"), 3);
        assert_eq!(solve_captcha("1111"), 4);
        assert_eq!(solve_captcha("1234"), 0);
        assert_eq!(solve_captcha("91212129"), 9);
    }

    #[test]
    fn test_solve_captcha_2() {
        assert_eq!(solve_captcha_2("1212"), 6);
        assert_eq!(solve_captcha_2("1221"), 0);
        assert_eq!(solve_captcha_2("123425"), 4);
        assert_eq!(solve_captcha_2("123123"), 12);
        assert_eq!(solve_captcha_2("12131415"), 4);
    }

    #[test]
    fn test_solve_first() {
        assert_eq!(
            solve(Path::new("./inputs/day_one.txt")).unwrap(),
            (1069, 1268)
        );
    }
}
