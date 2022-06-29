use std::collections::HashSet;

fn redistribute(banks: &mut [usize], index_to_redistribute: usize) {
    let mut redistribute_value = banks[index_to_redistribute];
    banks[index_to_redistribute] = 0;

    let mut index = index_to_redistribute + 1;
    loop {
        if redistribute_value == 0 {
            break;
        }

        if index == banks.len() {
            index = 0;
        }

        banks[index] += 1;

        redistribute_value -= 1;
        index += 1;
    }
}

fn max_index(xs: &[usize]) -> usize {
    let mut max_value = 0;
    let mut max_index = 0;

    for (i, x) in xs.iter().enumerate() {
        if *x > max_value {
            max_value = *x;
            max_index = i;
        }
    }

    max_index
}

fn solve_first(input: &str) -> usize {
    let mut values: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut value_set: HashSet<String> = HashSet::new();

    value_set.insert(format!("{:?}", values));

    let mut counter = 1;
    loop {
        let i = max_index(&values);
        redistribute(&mut values, i);

        if value_set.contains(&format!("{:?}", values)) {
            return counter;
        }

        value_set.insert(format!("{:?}", values));

        counter += 1;
    }
}


fn solve_second(input: &str) -> usize {
    let mut values: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut value_set: HashSet<String> = HashSet::new();
    let mut seen_before = false;

    value_set.insert(format!("{:?}", values));

    let mut counter = 1;
    loop {
        let i = max_index(&values);
        redistribute(&mut values, i);

        let results = (seen_before, value_set.contains(&format!("{:?}", values)));

        if results.0 && results.1 {
            return counter;
        }
        if !results.0 && results.1 {
            counter = 0;
            value_set = HashSet::new();
            seen_before = true;
        }

        value_set.insert(format!("{:?}", values));

        counter += 1;
    }
}

#[test]
fn test_redistribute() {
    let mut values = [0, 2, 7, 0];
    redistribute(&mut values, 2);
    assert!(values.iter().eq([2, 4, 1, 2].iter()))
}

#[test]
fn test_max_index() {
    assert_eq!(1, max_index(&[1, 5, 3, 4]));
    assert_eq!(4, max_index(&[1, 5, 3, 4, 6]));
}

#[test]
fn solve_sample() {
    assert_eq!(5, solve_first("0 2 7 0"));
}

#[test]
fn solve_sample_second() {
    assert_eq!(4, solve_second("0 2 7 0"));
}

#[test]
fn get_first_solution() {
    assert_eq!(12841, solve_first(INPUT));
}

#[test]
fn get_second_solution() {
    assert_eq!(8038, solve_second(INPUT));
}

const INPUT: &str = "4	10	4	1	8	4	9	14	5	1	14	15	0	15	3	5";
