// Inclusion-exclusion principle

fn count_squarefrees(bound: i64) -> i64 {
    primal::Primes::all()
        .map(|p| (p * p) as i64)
        .take_while(|&p| p < bound)
        .fold((bound - 1, vec![1i64]), |(mut sum, prods), p| {
            let mut next = Vec::new();
            for prod in prods {
                if prod
                    .checked_mul(p)
                    .map(|x| x.abs() >= bound)
                    .unwrap_or(true)
                {
                    sum += bound / prod;
                } else {
                    next.push(prod);
                    next.push(-p * prod);
                }
            }
            (sum, next)
        })
        .0
}

pub fn euler193() -> String {
    count_squarefrees(2i64.pow(50)).to_string()
}
