use num::pow::Pow;

fn the_poly(x: f64) -> f64 {
    (0..=10).map(|n| (-x).pow(n)).sum()
}

fn lagrange_basis(n: usize, coeffs: &Vec<f64>, x: f64) -> f64 {
    let mut new_coeffs = coeffs.clone();
    let coeff_n = new_coeffs.remove(n);
    new_coeffs.iter().map(|&coeff| x - coeff).product::<f64>()
        / new_coeffs
            .iter()
            .map(|&coeff| coeff_n - coeff)
            .product::<f64>()
    // new_coeffs.iter().map(|&coeff| (x - coeff) / (coeff_n - coeff)).product()
    // misses the answer by 1 due to floating arithmetic errors
}

fn lagrange_polynomial(values: &Vec<(f64, f64)>, x: f64) -> f64 {
    let (xs, ys): (Vec<f64>, Vec<f64>) = values.clone().into_iter().unzip();
    ys.into_iter()
        .zip(0..)
        .map(|(y, n)| y * lagrange_basis(n, &xs, x))
        .sum()
}

fn value(n: i64) -> i64 {
    lagrange_polynomial(
        &(1..=n).map(|k| (k as f64, the_poly(k as f64))).collect(),
        (n + 1) as f64,
    )
    .floor() as i64
}

pub fn euler101() -> String {
    (1..=10).map(value).sum::<i64>().to_string()
}
