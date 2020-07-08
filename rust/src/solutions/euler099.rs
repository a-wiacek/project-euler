use crate::utils::input::get_input;
use itertools::Itertools;
use ordered_float::OrderedFloat;

pub fn euler099() -> String {
    (1..)
        .zip(get_input(99).lines().map(|line| {
            let (base_str, exp_str) = line.split(',').take(2).collect_tuple().unwrap();
            OrderedFloat(exp_str.parse::<f64>().unwrap() * base_str.parse::<f64>().unwrap().ln())
        }))
        .max_by_key(|&(_, pow)| pow)
        .unwrap()
        .0
        .to_string()
}
