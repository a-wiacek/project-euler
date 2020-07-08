// Let b = 2 x, then 5x^2 (+-) 4x + 1 = L^2
// https://www.alpertron.com.ar/QUAD.HTM

pub fn euler138() -> String {
    (1..)
        .scan((0i64, 1i64), |state, _| {
            let (x, y) = *state;
            let ans = Some(y.abs());
            *state = (-9 * x - 4 * y - 4, -20 * x - 9 * y - 8);
            ans
        })
        .skip(1)
        .take(12)
        .sum::<i64>()
        .to_string()
}
