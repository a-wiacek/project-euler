use itertools::Itertools;

const SQUARES: [usize; 9] = [1, 4, 9, 16, 25, 36, 49, 64, 81];

fn rev(n: &usize) -> Vec<usize> {
    if [6, 9].contains(n) {
        vec![6, 9]
    } else {
        vec![*n]
    }
}

fn check(dice1: &Vec<usize>, dice2: &Vec<usize>) -> bool {
    let digits1: Vec<usize> = dice1.iter().flat_map(rev).collect();
    let digits2: Vec<usize> = dice2.iter().flat_map(rev).collect();
    let numbers: Vec<usize> = digits1
        .into_iter()
        .cartesian_product(digits2.into_iter())
        .flat_map(|(a, b)| vec![10 * a + b, 10 * b + a])
        .collect();
    SQUARES.iter().all(|s| numbers.contains(s))
}

pub fn euler090() -> String {
    ((0..10)
        .combinations(6)
        .cartesian_product((0..10).combinations(6))
        .filter(|(a, b)| check(a, b))
        .count()
        / 2)
    .to_string()
}
