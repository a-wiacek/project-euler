fn f(x: f64) -> f64 {
    let u = (x - 1.0).sqrt();
    u.atan() * (x + 1.0) - (1.0 / u).atan() * (x - 1.0) - u - u
}

fn expected(n: u32) -> f64 {
    let k = n as f64;
    (f((k + 0.5).powi(2)) - f((k - 0.5).powi(2))) / (k + k)
}

pub fn euler285() -> String {
    let expected_1_to_10 = 10.20914;
    format!(
        "{:.5}",
        expected_1_to_10 + (11..=100000).map(expected).sum::<f64>()
    )
}
