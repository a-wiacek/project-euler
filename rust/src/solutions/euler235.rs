fn f(r: f64) -> f64 {
    let s = r.powi(5000);
    300.0 * (s - 1.0) * (r - 1.0) - s * (r * 5000.0 - 5001.0) - 1.0
        + (r - 1.0).powi(2) * 200000000000.0
}

pub fn euler235() -> String {
    format!(
        "{:.12}",
        crate::utils::numeric::bisection::bisection(f, 1.001, 1.003, 40)
    )
}
