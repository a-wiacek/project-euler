use crate::utils::itertools::*;
use crate::utils::numeric::digits::undigits;
use primal::is_prime;

fn repeatable(n: usize, d: usize, d_count: usize) -> Vec<usize> {
    let digits_to_choose: Vec<usize> = (0..=9).filter(|&x| x != d).collect();
    let mut ans = Vec::new();
    for (d_positions, remaining_positions) in (0..n).double_combinations(d_count) {
        for remaining_ds in (0..n - d_count)
            .map(|_| digits_to_choose.clone())
            .multi_cartesian_product()
        {
            let mut digits = Vec::new();
            for i in 0..remaining_ds.len() {
                digits.push((remaining_positions[i], remaining_ds[i]));
            }
            for &pos in &d_positions {
                digits.push((pos, d));
            }
            digits.sort();
            let digits: Vec<usize> = digits.into_iter().map(|x| x.1).collect();
            if digits[0] != 0 {
                ans.push(undigits(&digits));
            }
        }
    }
    ans
}

fn repeatable_primes(n: usize, d: usize, d_count: usize) -> Vec<usize> {
    repeatable(n, d, d_count)
        .into_iter()
        .filter(|&p| is_prime(p as u64))
        .collect()
}

fn fun_s(n: usize, d: usize) -> usize {
    (0..=n)
        .rev()
        .map(|d_count| repeatable_primes(n, d, d_count))
        .find(|ans| !ans.is_empty())
        .unwrap()
        .into_iter()
        .sum()
}

fn sum_fun_s(n: usize) -> usize {
    (0..10).map(|d| fun_s(n, d)).sum()
}

pub fn euler111() -> String {
    sum_fun_s(10).to_string()
}
