use crate::utils::numeric::digits::Digits;

fn is_bouncy(n: usize) -> bool {
    let digits: Vec<usize> = n.digits();
    let mut not_inc = false;
    let mut not_dec = false;
    for i in 1..digits.len() {
        not_inc = not_inc || digits[i] > digits[i - 1];
        not_dec = not_dec || digits[i] < digits[i - 1];
    }
    not_inc && not_dec
}

pub fn euler112() -> String {
    let mut n = 99;
    let mut bouncy = 0;
    while 100 * bouncy != 99 * n {
        if is_bouncy(n + 1) {
            bouncy += 1;
        }
        n += 1;
    }
    n.to_string()
}
