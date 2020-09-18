fn f(s: u128, bound: u128) -> u128 {
    (bound - 2) / (2..=s).fold(1, num::integer::lcm)
}

fn p(s: u128, bound: u128) -> u128 {
    f(s, bound) - f(s + 1, bound)
}

pub fn euler601() -> String {
    (1u32..32)
        .map(|i| p(i as u128, 4u128.pow(i)))
        .sum::<u128>()
        .to_string()
}
