use crate::utils::number_theory::{chinese_remainder::chinese, totient::totient_array};

pub fn euler531() -> String {
    let lower_bound = 1_000_000;
    let bound = 1_005_000;
    let tots: Vec<i64> = totient_array(bound);
    let g = |a, m, b, n| chinese(a, m, b, n).unwrap_or(0);
    let f = |m, n| g(tots[n - 1], n as i64, tots[m - 1], m as i64);
    let mut ans = 0;
    for n in lower_bound..bound {
        for m in n + 1..bound {
            ans += f(n, m);
        }
    }
    ans.to_string()
}
