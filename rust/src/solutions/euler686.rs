pub fn euler686() -> String {
    let l10 = 10f64.log2();
    let l123 = 123f64.log2();
    let l124 = 124f64.log2();
    let find_power = |k: f64| {
        let s = k * l10;
        let low = (l123 + s).ceil();
        let high = (l124 + s).floor();
        if low == high {
            Some(high)
        } else {
            None
        }
    };
    (2u64..)
        .filter_map(|k| find_power(k as f64))
        .nth(678909)
        .unwrap()
        .to_string()
}
