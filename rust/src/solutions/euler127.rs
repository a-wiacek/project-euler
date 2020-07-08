use crate::utils::number_theory::radical::radical_array;

pub fn euler127() -> String {
    let bound = 120000 - 1;
    let radical_array = radical_array::<usize>(bound);
    let rad = |n| radical_array[n - 1];
    let mut ans = 0;
    for c in 1..bound {
        if c / rad(c) < 2 {
            continue;
        }
        for a in (1..=(c - 1) / 2).filter(|&a| rad(a) < c / rad(c)) {
            let b = c - a;
            // Since a, b and c are coprime, rad(abc) = rad(a) * rad(b) * rad(c)F
            if num::integer::gcd(a, b) == 1 && rad(a) * rad(b) * rad(c) < c {
                ans += c;
            }
        }
    }
    ans.to_string()
}
