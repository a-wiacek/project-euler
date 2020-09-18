use num::integer::{gcd, Roots};

fn f(a: u16, b: u16, c: u16, d: u16) -> bool {
    let area2 = (a + c) * (b + d);
    let bound_points = gcd(a, b) + gcd(b, c) + gcd(c, d) + gcd(d, a);
    let x = (area2 + 2 - bound_points) / 2;
    x.sqrt().pow(2) == x
}

fn compute(m: u16) -> usize {
    let mut ans = 0;
    for a in 1..=m {
        for b in 1..=m {
            for c in 1..=m {
                for d in 1..=m {
                    if f(a, b, c, d) {
                        ans += 1;
                    }
                }
            }
        }
    }
    ans
}

pub fn euler504() -> String {
    compute(100).to_string()
}
