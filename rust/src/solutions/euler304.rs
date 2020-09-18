use crate::utils::numeric::linear_recurrence_relation::LRR;

pub fn euler304() -> String {
    let p = 1234567891011u64;
    let fib = LRR::<u128>::new(vec![0, 1], vec![1, 1]);
    let mut fib1 = fib.at_mod(10usize.pow(14), p as u128) as u64;
    let mut fib2 = fib.at_mod(10usize.pow(14) + 1, p as u128) as u64;
    (10u64.pow(14)..)
        .map(|n| {
            let ans = (n, fib1);
            let tmp = (fib1 + fib2) % p;
            fib1 = fib2;
            fib2 = tmp;
            ans
        })
        .filter(|p| primal::is_prime(p.0))
        .take(100000)
        .map(|t| t.1)
        .sum::<u64>()
        .rem_euclid(p)
        .to_string()
}
