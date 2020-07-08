use crate::utils::numeric::digits::Digits;

pub fn euler125() -> String {
    let mut palindromic: Vec<u64> = Vec::new();
    let sum_of_squares: Vec<u64> = (0..=10000).map(|n| n * (n + 1) * (2 * n + 1) / 6).collect();
    for a in 2..=10000 {
        for b in 0..a - 1 {
            let k = sum_of_squares[a] - sum_of_squares[b];
            if k.is_palindrome() && k < 100_000_000 {
                palindromic.push(k);
            }
        }
    }
    palindromic.sort();
    palindromic.dedup();
    palindromic.into_iter().sum::<u64>().to_string()
}
