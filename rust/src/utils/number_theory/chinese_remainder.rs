use crate::utils::number_theory::invert_mod::InvertMod;
use num::{integer::ExtendedGcd, Integer};

// Given two pairs (a, m) and (b, n), find a number x
// such that 0 <= x < m * n, x % m = a and x % n = b.
// Numbers n and m can have common factors.

pub fn chinese<T>(a: T, m: T, b: T, n: T) -> Option<T>
where
    T: Integer + Clone,
{
    let ExtendedGcd { gcd, x, y, .. } = m.extended_gcd(&n);
    let rem = |a: T, b: T| {
        let ans = a % b.clone();
        if ans < T::zero() {
            ans + b
        } else {
            ans
        }
    };
    if gcd.is_one() {
        Some(rem(n.clone() * a * y + m.clone() * b * x, m * n))
    } else if (a.clone() - b.clone()).is_multiple_of(&gcd) {
        Some(rem(
            n.clone() / gcd.clone() * a * y + m.clone() / gcd.clone() * b * x,
            m / gcd * n,
        ))
    } else {
        None
    }
}

// Given a set of pairs (a_1, n_1), ..., (a_k, n_k),
// find a number x such that 0 <= x < n_1 * ... * n_k
// and x % n_i = a_i for 1 <= i <= n.
// If n_1, ..., n_k are not pairwise coprime,
// function will return None regardless of
// whether any solution exists.

pub fn chinese_remainder<T>(input: &[(T, T)]) -> Option<T>
where
    T: Integer + Clone,
{
    let product = input
        .iter()
        .map(|p| p.1.clone())
        .fold(T::one(), |a, b| a * b);
    let mut sum = T::zero();
    for (a, n) in input.iter() {
        let p = product.clone() / n.clone();
        sum = sum + a.clone() * p.invert_mod(n)? * p;
    }
    Some(sum % product)
}
