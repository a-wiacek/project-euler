use crate::utils::input::get_input;
use crate::utils::numeric::digits::undigits;
use std::collections::{HashMap, HashSet};

pub fn euler079() -> String {
    let mut digits_before: HashMap<usize, HashSet<usize>> = HashMap::new();
    for &d in &[0, 1, 2, 3, 6, 7, 8, 9] {
        // digits 4 and 5 do not exist
        digits_before.insert(d, HashSet::new());
    }
    for line in get_input(79).lines() {
        if let [d1, d2, d3] = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>()[..]
        {
            let before3 = digits_before.entry(d3).or_insert(HashSet::new());
            before3.insert(d1);
            before3.insert(d2);
            let before2 = digits_before.entry(d2).or_insert(HashSet::new());
            before2.insert(d1);
        }
    }
    let mut digits_before: Vec<(usize, usize)> = digits_before
        .into_iter()
        .map(|(i, before)| (i, before.len()))
        .collect();
    digits_before.sort_by_key(|x| x.1);
    undigits(&digits_before.into_iter().map(|x| x.0).collect()).to_string()
}
