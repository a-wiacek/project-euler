use crate::utils::number_theory::radical::radical_array;
use itertools::Itertools;

pub fn euler124() -> String {
    radical_array::<usize>(100000)
        .into_iter()
        .zip(1..)
        .sorted()
        .nth(9999)
        .unwrap()
        .1
        .to_string()
}
