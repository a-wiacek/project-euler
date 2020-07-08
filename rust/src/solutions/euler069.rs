use crate::utils::number_theory::totient::totient_array;
use ordered_float::OrderedFloat;
use std::ops::Add;

pub fn euler069() -> String {
    let tot: Vec<u32> = totient_array(1000001);
    (0usize..=1000000)
        .max_by_key(|&n| OrderedFloat((n as f64 + 1.0) / (tot[n] as f64)))
        .unwrap()
        .add(1)
        .to_string()
}
