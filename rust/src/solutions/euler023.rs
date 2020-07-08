use crate::utils::numeric::divisors::divisors;

pub fn euler023() -> String {
    let bound: usize = 28124;
    let abdunant: Vec<usize> = (1..bound)
        .filter(|&n| divisors(n).into_iter().sum::<usize>() > n << 1)
        .collect();
    let mut possible: Vec<bool> = vec![false; bound];
    for i in &abdunant {
        for j in &abdunant {
            let s = i + j;
            if s < bound {
                possible[s] = true;
            }
        }
    }
    possible
        .into_iter()
        .zip(0..)
        .filter_map(|(t, i)| if t { None } else { Some(i) })
        .sum::<usize>()
        .to_string()
}
