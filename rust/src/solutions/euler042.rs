use crate::utils::input::get_input;
use crate::utils::numeric::sequences::triangular::Triangular;

fn word_value(word: &str) -> usize {
    word.chars().map(|c| c as usize - 'A' as usize + 1).sum()
}

fn is_triangle_word(word: &&str) -> bool {
    Triangular::new()
        .take(20)
        .collect::<Vec<usize>>()
        .contains(&word_value(*word))
}

pub fn euler042() -> String {
    get_input(42)
        .split(',')
        .map(|l| l.trim_matches('"'))
        .filter(is_triangle_word)
        .count()
        .to_string()
}
