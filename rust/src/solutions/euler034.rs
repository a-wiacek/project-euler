use crate::utils::numeric::digits::Digits;
use crate::utils::numeric::factorial::factorial;

pub fn euler034() -> String {
    (3usize..100000)
        .filter(|&n| n.digits().into_iter().map(|n| factorial(&n)).sum::<usize>() == n)
        .sum::<usize>()
        .to_string()
}
