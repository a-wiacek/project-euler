use crate::utils::numeric::sequences::continued_fractions::square_root::SquareRoot;

pub fn euler064() -> String {
    (2..10000)
        .filter(|&n| SquareRoot::new(n, false).count() % 2 == 0)
        .count()
        .to_string()
}
