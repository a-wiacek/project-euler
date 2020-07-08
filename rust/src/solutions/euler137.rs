use itertools::Itertools;
use std::iter::successors;

pub fn euler137() -> String {
    let base_pairs: Vec<(i128, i128)> = vec![(0, 1), (2, 5), (0, -1), (-1, 2), (-1, -2)];
    successors(Some(base_pairs), |pairs| {
        Some(
            pairs
                .iter()
                .flat_map(|&(x, y)| {
                    vec![
                        (-9 * x - 4 * y - 2, -20 * x - 9 * y - 4),
                        (-9 * x + 4 * y - 2, 20 * x - 9 * y + 4),
                    ]
                })
                .collect(),
        )
    })
    .take(15)
    .flatten()
    .filter(|x| x.0 > 0)
    .sorted_by_key(|x| x.0)
    .unique_by(|x| x.0)
    .nth(14)
    .unwrap()
    .0
    .to_string()
}
