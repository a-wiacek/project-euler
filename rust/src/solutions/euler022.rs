use crate::utils::input::get_input;

fn word_value(word: &str) -> i64 {
    word.chars().map(|c| c as i64 - 'A' as i64 + 1).sum()
}

pub fn euler022() -> String {
    let mut array: Vec<String> = get_input(22)
        .split(',')
        .map(|l| l.trim_matches('"').to_string())
        .collect();
    array.sort();
    array
        .iter()
        .map(|w| word_value(&w))
        .zip(1..)
        .map(|(v, i)| v * i)
        .sum::<i64>()
        .to_string()
}
