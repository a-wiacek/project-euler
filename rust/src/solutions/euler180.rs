use num::rational::Ratio;

// f_n(x, y, z) = (x^n + y^n - z^n)(x + y + z)
// By Fermat's Last Theorem, solutions exist only if n \in \{-2, -1, 1, 2}.

pub fn euler180() -> String {
    let mut vec = Vec::<Ratio<u64>>::new();
    let fractions = (2u64..36)
        .flat_map(|b| (1..b).map(move |a| (a, b)))
        .filter(|(a, b)| num::Integer::gcd(a, b) == 1)
        .map(|(a, b)| Ratio::new_raw(a, b))
        .collect::<Vec<Ratio<u64>>>();
    let _1 = Ratio::new_raw(1, 1);
    for &x in &fractions {
        for &y in &fractions {
            if x <= y {
                for &z in &fractions {
                    if x * x + y * y == z * z
                        || x + y == z
                        || _1 / x + _1 / y == _1 / z
                        || _1 / (x * x) + _1 / (y * y) == _1 / (z * z)
                    {
                        vec.push(x + y + z);
                    }
                }
            }
        }
    }
    vec.sort();
    vec.dedup();
    let sum = vec.into_iter().sum::<Ratio<u64>>();
    (sum.numer() + sum.denom()).to_string()
}
