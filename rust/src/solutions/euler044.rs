use crate::utils::numeric::sequences::pentagonal::Pentagonal;
use std::collections::HashSet;

pub fn euler044() -> String {
    let pentagonals: Vec<i32> = Pentagonal::new().take(2500).collect();
    let mut ans = 99999999;
    let mut pentagonals_set = HashSet::new();
    for p in &pentagonals {
        pentagonals_set.insert(p);
    }
    for p in &pentagonals {
        for q in &pentagonals {
            if p > q && pentagonals_set.contains(&(p - q)) && pentagonals_set.contains(&(p + q)) {
                ans = std::cmp::min(ans, p - q);
            }
        }
    }
    ans.to_string()
}
