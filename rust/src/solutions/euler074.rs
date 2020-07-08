use crate::utils::numeric::digits::Digits;
use crate::utils::numeric::factorial::factorial;
use std::collections::HashSet;

fn cycle_length(mut n: usize) -> usize {
    let mut prevs = HashSet::new();
    prevs.insert(n);
    loop {
        n = n.digits().iter().map(factorial).sum();
        if prevs.contains(&n) || prevs.len() > 60 {
            break prevs.len();
        }
        prevs.insert(n);
    }
}

pub fn euler074() -> String {
    (1..1000000)
        .filter(|&n| cycle_length(n) == 60)
        .count()
        .to_string()
}
