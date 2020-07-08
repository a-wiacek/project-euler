use crate::utils::numeric::digits::Digits;

pub fn euler052() -> String {
    (1..)
        .find(|&n| {
            let ds: Vec<Vec<usize>> = (1..=6).map(|i| (n * i).digits_count()).collect();
            (1..=5).all(|i| ds[0] == ds[i])
        })
        .unwrap()
        .to_string()
}
