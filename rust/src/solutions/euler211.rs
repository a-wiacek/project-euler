use num::integer::Roots;

pub fn euler211() -> String {
    let bound = 64_000_000;
    let mut divs = vec![0; bound];
    for d in 1..bound.sqrt() {
        let d2 = d * d;
        divs[d2] += d2;
        for b in (d2 + d..bound).step_by(d) {
            let bd = b / d;
            divs[b] += d2 + bd * bd;
        }
    }
    divs.into_iter()
        .enumerate()
        .filter(|x| x.1.sqrt().pow(2) == x.1)
        .map(|x| x.0)
        .sum::<usize>()
        .to_string()
}
