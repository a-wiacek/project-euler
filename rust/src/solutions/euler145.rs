use crate::utils::numeric::digits::{undigits, Digits};

pub fn euler145() -> String {
    // Numbers with number of digits equal to 1 modulo 4 can't satisfy the property
    (1u32..100_000_000)
        .filter(|&n| n % 10 != 0)
        .filter(|&n| {
            let mut rev_digits = n.digits();
            rev_digits.reverse();
            let sum_digits = (n + undigits(&rev_digits)).digits();
            sum_digits.into_iter().all(|d| d % 2 == 1)
        })
        .count()
        .to_string()
}
