fn area(n: u32) -> f64 {
    let n = n as f64;
    let z = (n + n).sqrt();
    let k = n + z + 1.0;
    let area = (z + 2.0) / 2.0 / k - 0.5 * ((z + 1.0) / k).asin();
    area / (1.0 - std::f64::consts::PI / 4.0)
}

pub fn euler587() -> String {
    (1..).find(|&n| area(n) < 0.001).unwrap().to_string()
}
