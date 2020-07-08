type P = (i64, i64, i64);

// https://en.wikipedia.org/wiki/Tree_of_primitive_Pythagorean_triples
fn matrix_a((a, b, c): P) -> P {
    (a - 2 * b + 2 * c, 2 * a - b + 2 * c, 2 * a - 2 * b + 3 * c)
}

fn matrix_b((a, b, c): P) -> P {
    (a + 2 * b + 2 * c, 2 * a + b + 2 * c, 2 * a + 2 * b + 3 * c)
}

fn matrix_c((a, b, c): P) -> P {
    (
        -a + 2 * b + 2 * c,
        -2 * a + b + 2 * c,
        -2 * a + 2 * b + 3 * c,
    )
}

const BOUND: i64 = 100000000 - 1;

fn count(t: P) -> i64 {
    let (a, b, c) = t;
    let per = a + b + c;
    if per > BOUND {
        0
    } else {
        let s = if c % (b - a) == 0 { BOUND / per } else { 0 };
        vec![count(matrix_a(t)), count(matrix_b(t)), count(matrix_c(t))]
            .into_iter()
            .sum::<i64>()
            + s
    }
}

pub fn euler139() -> String {
    count((3, 4, 5)).to_string()
}
