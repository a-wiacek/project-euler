use crate::utils::numeric::digits::Digits;
use itertools::Itertools;

pub fn euler348() -> String {
    let mut ans = 0;
    let mut palindromes = Vec::<u64>::new();
    for a in 2..=40000 {
        for b in 2..=4000 {
            let p = a * a + b * b * b;
            if p.is_palindrome() {
                palindromes.push(p);
            }
        }
    }
    palindromes.sort();
    for (p, group) in &palindromes.into_iter().group_by(|&p| p) {
        if group.count() == 4 {
            ans += p;
        }
    }
    ans.to_string()
}
