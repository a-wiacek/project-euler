use crate::utils::number_theory::totient::totient_array;
use crate::utils::numeric::digits::Digits;
use ordered_float::OrderedFloat;
use std::ops::Add;

pub fn euler070() -> String {
    let tot: Vec<usize> = totient_array(10000000);
    (1usize..10000000)
        .filter(|&n| (n + 1).digits_count() == tot[n].digits_count())
        .min_by_key(|&n| OrderedFloat((n as f64 + 1.0) / (tot[n] as f64)))
        .unwrap()
        .add(1)
        .to_string()
}
