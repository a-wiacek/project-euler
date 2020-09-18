fn f(p: usize, q: usize, r: usize) -> usize {
    2 * p * q * r - p * q - q * r - r * p
}

pub fn euler278() -> String {
    let primes = primal::Sieve::new(5000)
        .primes_from(0)
        .take_while(|&p| p < 5000)
        .collect::<Vec<usize>>();
    let mut ans = 0;
    let l = primes.len();
    for i in 0..l {
        for j in i + 1..l {
            for k in j + 1..l {
                ans += f(primes[i], primes[j], primes[k]);
            }
        }
    }
    ans.to_string()
}
