pub fn euler085() -> String {
    let (best_i, best_j) = (1i64..=2000)
        .flat_map(|i| (i..=2000).map(move |j| (i, j)))
        .min_by_key(|&(i, j)| (i * (i + 1) * j * (j + 1) / 4 - 2000000).abs())
        .unwrap();
    (best_i * best_j).to_string()
}
