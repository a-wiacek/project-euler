use num::integer::gcd;

fn unconcealed(p: i64, q: i64) -> i64 {
    let phi = (p - 1) * (q - 1);
    (2..phi)
        .filter(|&e| gcd(e, phi) == 1)
        .filter(|&e| gcd(e - 1, p - 1) == 2)
        .filter(|&e| gcd(e - 1, q - 1) == 2)
        .sum()
}

pub fn euler182() -> String {
    unconcealed(1009, 3643).to_string()
}
