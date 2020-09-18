fn pows(p: usize) -> impl Iterator<Item = usize> {
    std::iter::successors(Some(1usize), move |x| x.checked_mul(p))
}

fn hamming(typ: usize, bound: usize) -> usize {
    let (count, finale) = primal::Primes::all().take_while(|&p| p <= typ).fold(
        (0usize, vec![1usize]),
        |(mut count, curr), p| {
            let mut next = Vec::new();
            for n in curr {
                if n * p > bound {
                    count += 1;
                } else {
                    next.extend(pows(p).map(|x| x * n).take_while(|&x| x <= bound));
                }
            }
            (count, next)
        },
    );
    count + finale.len()
}

pub fn euler204() -> String {
    hamming(100, 1_000_000_000).to_string()
}
