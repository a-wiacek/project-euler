fn update((x, y): (u64, u64)) -> (u64, u64) {
    (2 * x + 3 * y, x + 2 * y)
}

const BOUND: u64 = 1000000000;

fn case1((x, y): (u64, u64)) -> u64 {
    let n = 4 * y * y + 1;
    let perimeter = 3 * n + 1;
    if perimeter > BOUND {
        0
    } else {
        perimeter + case1(update((x, y)))
    }
}

fn case2((x, y): (u64, u64)) -> u64 {
    let n = 2 * (x + 1) / 3 - 1;
    let perimeter = 3 * n - 1;
    if perimeter > BOUND {
        0
    } else {
        perimeter + case2(update(update((x, y))))
    }
}

pub fn euler094() -> String {
    (case1((2, 1)) + case2(update(update((2, 1))))).to_string()
}
