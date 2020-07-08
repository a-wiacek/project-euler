use crate::utils::numeric::digits::undigits;
use permutohedron::LexicalPermutation;

pub fn euler043() -> String {
    let mut v: Vec<i64> = vec![1, 0, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut ans = 0;
    loop {
        {
            let divs = |i, p| (100 * &v[i] + 10 * &v[i + 1] + &v[i + 2]) % p == 0;
            if divs(1, 2)
                && divs(2, 3)
                && divs(3, 5)
                && divs(4, 7)
                && divs(5, 11)
                && divs(6, 13)
                && divs(7, 17)
            {
                ans += undigits(&v);
            }
        }
        if !v.next_permutation() {
            break ans.to_string();
        }
    }
}
