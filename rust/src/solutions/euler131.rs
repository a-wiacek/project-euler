use std::cmp::Ordering;

fn common_sorted(xs: &[usize], ys: &[usize]) -> usize {
    if xs.is_empty() || ys.is_empty() {
        0
    } else {
        match xs[0].cmp(&ys[0]) {
            Ordering::Less => common_sorted(&xs[1..], ys),
            Ordering::Equal => 1 + common_sorted(&xs[1..], &ys[1..]),
            Ordering::Greater => common_sorted(xs, &ys[1..]),
        }
    }
}

pub fn euler131() -> String {
    let cubediffs: Vec<usize> = (1usize..)
        .map(|n| (n + 1).pow(3) - n.pow(3))
        .take_while(|&n| n <= 1000000)
        .collect();
    let primes: Vec<usize> = primal::Sieve::new(1000000).primes_from(0).collect();
    common_sorted(&cubediffs[..], &primes[..]).to_string()
}
