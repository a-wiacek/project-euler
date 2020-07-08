use crate::utils::numeric::digits::Digits;

fn add_pointiwse(v: Vec<usize>, w: Vec<usize>) -> Vec<usize> {
    v.into_iter()
        .zip(w.into_iter())
        .map(|(a, b)| a + b)
        .collect()
}

fn get_pandigital(n: usize, i: usize) -> Option<i64> {
    let prods = (1..=i).map(|x| n * x).collect::<Vec<usize>>();
    let digits_prod = prods
        .iter()
        .map(Digits::digits_count)
        .fold(vec![0usize; 10], add_pointiwse);
    if (0..10).all(|d| digits_prod[d] == if d > 0 { 1 } else { 0 }) {
        prods
            .into_iter()
            .map(|x| x.to_string())
            .collect::<String>()
            .parse::<i64>()
            .ok()
    } else {
        None
    }
}

fn get_any_pandigital(n: usize) -> Option<i64> {
    (2..6).map(|i| get_pandigital(n, i)).find_map(|x| x)
}

pub fn euler038() -> String {
    (1..10000)
        .filter_map(get_any_pandigital)
        .max()
        .unwrap()
        .to_string()
}
