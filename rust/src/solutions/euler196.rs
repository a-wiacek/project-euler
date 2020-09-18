use itertools::iproduct;
use primal::is_prime;

// n -> rows (n - 2..=n + 2)
fn rows(n: u64) -> Vec<Vec<u64>> {
    vec![
        ((n - 2) * (n - 3) / 2 + 1..=(n - 1) * (n - 2) / 2).collect(),
        ((n - 1) * (n - 2) / 2 + 1..=n * (n - 1) / 2).collect(),
        (n * (n - 1) / 2 + 1..=(n + 1) * n / 2).collect(),
        ((n + 1) * n / 2 + 1..=(n + 2) * (n + 1) / 2).collect(),
        ((n + 2) * (n + 1) / 2 + 1..=(n + 3) * (n + 2) / 2).collect(),
    ]
}

fn neighbours(x: u64, y: u64) -> impl Iterator<Item = (u64, u64)> {
    iproduct!(
        std::cmp::max(x, 1) - 1..=std::cmp::min(x, 3) + 1,
        std::cmp::max(y, 1) - 1..=y + 1
    )
}

fn count(n: u64) -> u64 {
    let rows = rows(n);
    let primes: Vec<Vec<bool>> = rows
        .iter()
        .map(|row| row.iter().map(|&p| is_prime(p)).collect())
        .collect();
    let is_prime = |x: u64, y: u64| *primes[x as usize].get(y as usize).unwrap_or(&false);
    let mut marked: Vec<Vec<bool>> = rows.iter().map(|row| vec![false; row.len()]).collect();
    for x in 0..5 {
        for y in 0..n + 2 {
            if is_prime(x, y) {
                let prime_nbors = neighbours(x, y)
                    .filter(|&(x, y)| is_prime(x, y))
                    .collect::<Vec<_>>();
                if prime_nbors.len() >= 3 {
                    for (x, y) in prime_nbors {
                        marked[x as usize][y as usize] = true;
                    }
                }
            }
        }
    }
    let mut ans = 0;
    for y in 0..n as usize {
        if marked[2][y] {
            ans += rows[2][y];
        }
    }
    ans
}

pub fn euler196() -> String {
    (count(5678027) + count(7208785)).to_string()
}
