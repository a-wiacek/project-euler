// Number of pairs (x, y) with properties:
// 1) x + y = (12017639147 + 3) / 2 = 6008819575
// 2) x == y mod 3 -> x == y == 2 mod 3
// 3) gcd x y == 1

fn solutions(_n: usize) -> usize {
    let n = (_n + 3) / 2;
    (2..=n)
        .step_by(3)
        .filter(|&m| num::integer::gcd(m, n) == 1)
        .count()
}

pub fn euler202() -> String {
    solutions(12017639147).to_string()
}
