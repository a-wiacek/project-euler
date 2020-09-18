fn s(n: u64) -> u64 {
    let mut ans = 0;
    for beta in 1u64..=(n as f64).powf(0.25).floor() as u64 {
        for alpha in (1..=beta).filter(|&alpha| num::integer::gcd(alpha, beta) == 1) {
            let alpha2 = alpha * alpha;
            let beta2 = beta * beta;
            let den = (alpha + beta) * (alpha + beta);
            let max_k = n / (beta2 * den);
            let ra_base = den * alpha2;
            let rb_base = den * beta2;
            let rc_base = alpha2 * beta2;
            ans += (ra_base + rb_base + rc_base) * max_k * (max_k + 1) / 2;
        }
    }
    ans
}

pub fn euler510() -> String {
    s(1_000_000_000).to_string()
}
