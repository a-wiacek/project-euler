use crate::utils::numeric::digits::Digits;

fn square_digits(n: usize) -> bool {
    match n {
        1 => false,
        89 => true,
        _ => square_digits(n.digits().into_iter().map(|d| d * d).sum()),
    }
}

pub fn euler092() -> String {
    (1..10000000)
        .filter(|&n| square_digits(n))
        .count()
        .to_string()
}
