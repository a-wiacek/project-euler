use crate::utils::numeric::divisors::divisors;
use crate::utils::numeric::sequences::triangular::Triangular;

pub fn euler012() -> String {
    Triangular::<i64>::new()
        .find(|&x| divisors(x).len() > 500)
        .unwrap()
        .to_string()
}
