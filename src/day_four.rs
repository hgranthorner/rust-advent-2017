pub fn validate_passphrase(phrase: &str) -> bool {
    let mut words = [""; 15];
    let mut counter = 0;

    for w in phrase.split_whitespace() {
        if words.contains(&w) {
            return false;
        }

        words[counter] = w;
        counter += 1;
    }

    true
}

pub fn validate_passphrase_2(phrase: &str) -> bool {
    let mut words: [String; 15] = Default::default();
    let mut counter = 0;

    for w in phrase.split_whitespace() {
        let mut sorted = w.chars().collect::<Vec<char>>();
        sorted.sort_unstable();
        let s: String = sorted.into_iter().collect();
        if words.contains(&s) {
            return false;
        }

        words[counter] = s;
        counter += 1;
    }

    true
}

#[test]
fn can_validate_passphrase() {
    assert!(validate_passphrase("aa bb"));
    assert!(!validate_passphrase("aa aa"));
}

#[test]
fn get_solution_one() -> std::io::Result<()> {
    let contents = std::fs::read_to_string("./inputs/day_four.txt")?;

    let mut counter = 0;

    for phrase in contents.lines() {
        if validate_passphrase(phrase) {
            counter += 1;
        }
    }

    assert_eq!(466, counter);

    Ok(())
}

#[test]
fn get_solution_two() -> std::io::Result<()> {
    let contents = std::fs::read_to_string("./inputs/day_four.txt")?;

    let mut counter = 0;

    for phrase in contents.lines() {
        if validate_passphrase_2(phrase) {
            counter += 1;
        }
    }

    assert_eq!(251, counter);

    Ok(())
}
