use cached::proc_macro::cached;
use itertools::iproduct;

#[cached]
fn count(length: usize, ls_used: usize, as_used: usize) -> usize {
    if length == 0 {
        if ls_used == 0 && as_used == 0 {
            1
        } else {
            0
        }
    } else {
        let p00 = count(length - 1, 0, 0);
        let p11 = count(length - 1, 1, 1);
        let p02 = count(length - 1, 0, 2);
        let p10 = count(length - 1, 1, 0);
        let p01 = count(length - 1, 0, 1);
        let p12 = count(length - 1, 1, 2);
        match (ls_used, as_used) {
            (0, 0) => p00 + p01 + p02,
            (0, 1) => p00,
            (0, 2) => p01,
            (1, 0) => p00 + p01 + p02 + p10 + p11 + p12,
            (1, 1) => p10,
            (1, 2) => p11,
            _ => panic!()
        }
    }
}

pub fn euler191() -> String {
    iproduct!(0..2, 0..3)
        .map(|(l, a)| count(30, l, a))
        .sum::<usize>()
        .to_string()
}
