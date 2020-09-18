use std::iter::successors;

fn number_of_solutions(p: usize) -> u32 {
    if p == 3 || p % 6 == 1 {
        1
    } else {
        0
    }
}

fn e(sols: u32) -> u32 {
    if sols == 5 {
        1
    } else {
        5 - sols
    }
}

pub fn euler272() -> String {
    let bound = 10usize.pow(11);
    let mut ans = 0;
    let mut prods = vec![(1usize, 0u32)];
    for p in primal::Primes::all() {
        if prods.is_empty() {
            break;
        }
        prods = prods
            .into_iter()
            .flat_map(|(product, sols)| {
                let cond = p.checked_pow(e(sols))
                    .and_then(|pe| pe.checked_mul(product))
                    .map(|cond| cond > bound)
                    .unwrap_or(true);
                if cond {
                    if sols == 5 {
                        ans += product;
                    }
                    vec![]
                } else {
                    let ss = number_of_solutions(p);
                    let first = if p == 3 && product * 3 <= bound {
                        vec![(product, sols), (product * 3, sols)]
                    } else {
                        vec![(product, sols)]
                    };
                    let later = successors(
                        Some((product * if p == 3 { 9 } else { p }, sols + ss))
                            .filter(|&(product, sols)| product <= bound && sols <= 5),
                        |&(product, sols)| product.checked_mul(p).map(|x| (x, sols)),
                    )
                    .filter(|(product, _)| product <= &bound);
                    first.into_iter().chain(later).collect()
                }
            })
            .collect();
    }
    ans.to_string()
}
