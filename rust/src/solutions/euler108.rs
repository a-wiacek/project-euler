use primal::Sieve;

// 1/x + 1/y = 1/n -> (x - n)(y - n) = n^2
// Let n^2 = p_1^(2a_1) .. p_k^(2a_k).
// Then equation has ((2a_1 + 1)..(2a_k + 1) + 1)/2 distinct solutions (x <= y)
// Smallest n has a1 >= a2 >= ..

fn process_loop(sieve: &Sieve, limit: u32, ans_bound: usize, exps: Vec<u32>, d: usize) -> usize {
    if d == exps.len() {
        let pr: u32 = exps
            .iter()
            .fold(1, |pr, n| pr.checked_mul(2 * n + 1).unwrap_or(2 * limit));
        if pr > limit && pr < 2 * limit {
            exps.into_iter()
                .zip(sieve.primes_from(0))
                .fold(1, |pr, (n, p)| {
                    pr.checked_mul(p.pow(n)).unwrap_or(ans_bound)
                })
        } else {
            ans_bound
        }
    } else {
        let l = if d == 0 { 3 } else { exps[d - 1] };
        (0..=l)
            .map(|i| {
                let mut new_exps = exps.clone();
                new_exps[d] = i;
                process_loop(sieve, limit, ans_bound, new_exps, d + 1)
            })
            .min()
            .unwrap_or(ans_bound)
    }
}

// Find the smallest n such that n^2 has more than d divisors
pub fn process(d: u32) -> usize {
    let bound = 15;
    let sieve = &Sieve::new(100);
    process_loop(
        sieve,
        2 * d,
        sieve.primes_from(0).take(bound).product(),
        vec![0; bound],
        0,
    )
}

pub fn euler108() -> String {
    process(1000).to_string()
}
