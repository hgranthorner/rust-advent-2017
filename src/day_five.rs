fn solve(input: &str) -> usize {
    let mut instructions: Vec<i32> = input.lines().map(|n| n.parse().unwrap()).collect();

    let mut counter = 0;
    let mut index = 0;

    loop {
        if index >= instructions.len() {
            return counter;
        }
        let instruction = instructions[index];
        instructions[index] = instruction + 1;
        counter += 1;
        index = (instruction + (index as i32)) as usize;
    }
}

fn solve_second(input: &str) -> usize {
    let mut instructions: Vec<i32> = input.lines().map(|n| n.parse().unwrap()).collect();

    let mut counter = 0;
    let mut index = 0;

    loop {
        if index >= instructions.len() {
            return counter;
        }
        let instruction = instructions[index];
        if instruction > 2 {
            instructions[index] = instruction - 1;
        } else {
            instructions[index] = instruction + 1;
        }

        counter += 1;
        index = (instruction + (index as i32)) as usize;
    }
}

#[test]
fn solve_first_sample() {
    assert_eq!(
        5,
        solve(
            "0
3
0
1
-3"
        )
    );
}

#[test]
fn solve_second_sample() {
    assert_eq!(
        10,
        solve_second(
            "0
3
0
1
-3"
        )
    );
}

#[test]
fn get_first_solution() -> std::io::Result<()> {
    let contents = std::fs::read_to_string("./inputs/day_five.txt")?;

    assert_eq!(372671, solve(&contents));
    Ok(())
}

#[test]
fn get_second_solution() -> std::io::Result<()> {
    let contents = std::fs::read_to_string("./inputs/day_five.txt")?;

    assert_eq!(25608480, solve_second(&contents));
    Ok(())
}
