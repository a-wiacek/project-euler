use num::bigint::BigInt;

pub fn euler013() -> String {
    let mut sum = crate::utils::input::get_input(13)
        .lines()
        .map(|x| x.parse::<BigInt>().unwrap())
        .sum::<BigInt>()
        .to_string();
    sum.truncate(10);
    sum
}
