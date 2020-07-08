use crate::utils::numeric::digits::Digits;

pub fn euler036() -> String {
    (1usize..1000000)
        .filter(|&n| n.is_palindrome())
        .filter(|&n| n.is_palindrome_in_radix(2))
        .sum::<usize>()
        .to_string()
}
