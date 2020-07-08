use num::integer::binomial;

fn f(n: usize, s: usize) -> usize {
    binomial(n, s) * binomial(n - s, s) / 2 - binomial(2 * s, s) * binomial(n, 2 * s) / (s + 1)
}

pub fn euler106() -> String {
    (2..=6).map(|s| f(12, s)).sum::<usize>().to_string()
}
