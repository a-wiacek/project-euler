use crate::utils::numeric::divisors::divisors;
use itertools::Itertools;

fn suffixes(n: usize) -> impl Iterator<Item = u128> {
    std::iter::once(1u128..3)
        .chain((0..n).map(|_| 0u128..3))
        .multi_cartesian_product()
        .map(|digits| digits.into_iter().fold(0u128, |x, y| 10 * x + y))
}

fn raw_compute(n: u128) -> u128 {
    (0..)
        .filter_map(|k| suffixes(k).find(|&u| u % n == 0))
        .next()
        .map(|d| d / n as u128)
        .unwrap()
}

fn compute(bound: u16) -> u128 {
    let mut g = vec![0u128; bound as usize + 1];
    g[1] = 1;
    for n in 2..=bound as usize {
        if n % 10 == 0 {
            g[n] = g[n / 10];
        } else {
            let divs = divisors(n);
            g[n] = divs[1..divs.len() - 1]
                .into_iter()
                .rev()
                .filter_map(|&div| {
                    let g = g[n / div];
                    if g % div as u128 == 0 {
                        Some(g / div as u128)
                    } else {
                        None
                    }
                })
                .next()
                .unwrap_or_else(|| raw_compute(n as u128));
        }
    }
    g.into_iter().sum()
}

pub fn euler303() -> String {
    compute(10000).to_string()
}
