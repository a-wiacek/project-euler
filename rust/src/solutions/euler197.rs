fn f(x: f64) -> f64 {
    2f64.powf(30.403243784 - x * x).floor() * 10f64.powi(-9)
}

pub fn euler197() -> String {
    let fixpoints = std::iter::successors(Some(-1.0), |&x| Some(f(x)))
        .map(|x| (x * 10f64.powi(9)).floor())
        .skip(600)
        .take(2)
        .collect::<Vec<f64>>();
    ((fixpoints[0] + fixpoints[1]) / 10f64.powi(9)).to_string()
}
