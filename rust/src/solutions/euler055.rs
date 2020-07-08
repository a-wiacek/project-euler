use crate::utils::numeric::digits::Digits;
use num::bigint::BigInt;

fn palindrome(n: &BigInt) -> BigInt {
    n.digits()
        .into_iter()
        .rev()
        .fold(BigInt::from(0), |a, b| 10 * a + BigInt::from(b))
}

fn lynchrel(mut n: BigInt) -> bool {
    for _ in 0..50 {
        n = &n + palindrome(&n);
        if n.is_palindrome() {
            return false;
        }
    }
    true
}

pub fn euler055() -> String {
    (1..10000)
        .filter(|&n| lynchrel(BigInt::from(n)))
        .count()
        .to_string()
}
