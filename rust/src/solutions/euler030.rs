use crate::utils::numeric::digits::Digits;

fn fifth(n: usize) -> bool {
    n == n.digits().into_iter().map(|n| n.pow(5)).sum()
}

pub fn euler030() -> String {
    (2..1000000)
        .filter(|&n| fifth(n))
        .sum::<usize>()
        .to_string()
}
