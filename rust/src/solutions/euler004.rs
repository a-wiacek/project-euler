use crate::utils::numeric::digits::Digits;

pub fn euler004() -> String {
    let mut max = 0;
    for m in 100..1000 {
        for n in m..1000 {
            let p = m * n;
            if p.is_palindrome() {
                max = std::cmp::max(max, p);
            }
        }
    }
    max.to_string()
}
