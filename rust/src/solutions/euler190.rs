pub fn euler190() -> String {
    (2..16)
        .map(|_m| {
            let m = _m as f64;
            ((2..=_m).map(|m| (m as f64).powi(m)).product::<f64>()
                * (2.0 / (m + 1.0)).powf(m * (m + 1.0) / 2.0)) as i128
        })
        .sum::<i128>()
        .to_string()
}
