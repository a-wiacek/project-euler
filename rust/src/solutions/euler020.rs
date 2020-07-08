use num::bigint::BigInt;

pub fn euler020() -> String {
    (1..=100)
        .map(BigInt::from)
        .product::<BigInt>()
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>()
        .to_string()
}
