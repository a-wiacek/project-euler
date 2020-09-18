use num::{integer::Roots, rational::Ratio};

fn best_approx(bound: u128, n: u128) -> (u128, u128) {
    let mut low = (0, 1);
    let mut high = (1, 0);
    let distance = |(p, q): (u128, u128)| {
        let x = Ratio::new(p, q).pow(2);
        let y = Ratio::from_integer(n);
        if x > y {
            x - y
        } else {
            y - x
        }
    };
    loop {
        let p = low.0 + high.0;
        let q = low.1 + high.1;
        if q > bound {
            return if distance(low) < distance(high) {
                low
            } else {
                high
            };
        } else {
            let mid = (p, q);
            if p * p > q * q * n {
                high = mid;
            } else {
                low = mid;
            }
        }
    }
}

pub fn euler192() -> String {
    let bound = 10u128.pow(12);
    (2u128..=100000)
        .filter(|&n| n.sqrt().pow(2) != n)
        .map(|n| best_approx(bound, n).1)
        .sum::<u128>()
        .to_string()
}
