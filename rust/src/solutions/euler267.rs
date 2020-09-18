use num::{integer::binomial, BigInt, ToPrimitive};

pub fn euler267() -> String {
    let num: f64 = (432..=1000)
        .map(|x| binomial(BigInt::from(1000), BigInt::from(x)))
        .sum::<BigInt>()
        .to_f64()
        .unwrap();
    let den = 2f64.powi(1000);
    format!("{:.12}", num / den)
}
