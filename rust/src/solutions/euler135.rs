use crate::utils::numeric::divisors::divisors;

pub fn euler135() -> String {
    (1..1000000)
        .filter(|&n| {
            divisors(n)
                .into_iter()
                .filter(|&y| {
                    // If x = y + d and z = y - d, then we have n = y(4y - d)
                    let d4 = n / y + y;
                    d4 % 4 == 0 && d4 < 4 * y
                })
                .count()
                == 10
        })
        .count()
        .to_string()
}
