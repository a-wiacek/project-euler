use itertools::Itertools;
use num::integer::binomial;

pub fn euler493() -> String {
    let rolls = (0..7)
        .map(|_| 0u128..=10)
        .multi_cartesian_product()
        .filter(|roll| roll.iter().sum::<u128>() == 20);
    let numer = rolls
        .map(|roll| {
            let non_zero_count = roll.iter().filter(|&&d| d != 0).count() as u128;
            let binoms = roll.into_iter().map(|k| binomial(10, k)).product::<u128>();
            non_zero_count * binoms
        })
        .sum::<u128>();
    let denom = binomial(70u128, 20);
    format!("{:.9}", numer as f64 / denom as f64)
}
