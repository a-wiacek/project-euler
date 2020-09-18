use crate::utils::numeric::digits::{undigits, Digits};

fn is_ok(n: u64) -> bool {
    undigits(&(n * n).digits().into_iter().step_by(2).collect()) == 123456789
}

pub fn euler206() -> String {
    let mut n = 10101010;
    loop {
        if is_ok(10 * n + 3) {
            break 100 * n + 30;
        } else if is_ok(10 * n + 7) {
            break 100 * n + 70;
        } else {
            n += 1;
        }
    }
    .to_string()
}
