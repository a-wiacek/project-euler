use crate::utils::number_theory::totient::totient_array;
use std::ops::Sub;

pub fn euler072() -> String {
    totient_array::<usize>(1000000)
        .into_iter()
        .sum::<usize>()
        .sub(1)
        .to_string()
}
