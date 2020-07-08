use itertools::Itertools;
use std::iter::successors;

pub fn euler140() -> String {
    let base_pairs: Vec<(i128, i128)> = vec![(2, -7), (0, -1), (0, 1), (-4, 5), (-3, 2), (-3, 2)];
    successors(Some(base_pairs), |pairs| {
        Some(
            pairs
                .iter()
                .flat_map(|&(x, y)| {
                    vec![
                        (-9 * x - 4 * y - 14, -20 * x - 9 * y - 28),
                        (-9 * x + 4 * y - 14, 20 * x - 9 * y + 28),
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
    .take(30)
    .map(|x| x.0)
    .sum::<i128>()
    .to_string()
}
