use crate::utils::numeric::digits::Digits;
use itertools::Itertools;

pub fn euler571() -> String {
    let base: usize = 12;
    (0..base)
        .permutations(base)
        .filter(|perm| perm[0] > 0)
        .map(|perm| perm.into_iter().fold(0, |num, d| num * base + d))
        .filter(|n| {
            (4..base).rev().all(|base| {
                n.digits_count_in_radix(base)
                    .into_iter()
                    .all(|count| count > 0)
            })
        })
        .take(10)
        .sum::<usize>()
        .to_string()
}
