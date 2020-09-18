use std::iter::successors;

fn f((x, y): (i128, i128)) -> (i128, i128) {
    (3 * x + 2 * y + 3, 4 * x + 3 * y + 5)
}

pub fn euler321() -> String {
    let g = |x, y| successors(Some((x, y)), |&next| Some(f(next))).skip(1);
    g(0, -1)
        .take(20)
        .chain(g(0, 0).take(20))
        .map(|pair| pair.0)
        .sum::<i128>()
        .to_string()
}
