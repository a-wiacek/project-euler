use itertools::iproduct;
// plug in 9 values, compute rest
// abcd
// efgh
// ijkl
// mnop

pub fn euler166() -> String {
    let mut ans = 0;
    let digitize = |d: i32| if d >= 0 && d < 10 { Some(d) } else { None };
    for (c, d, e, f, g, j, l, m) in
        iproduct!(0..10, 0..10, 0..10, 0..10, 0..10, 0..10, 0..10, 0..10)
    {
        let sum = d + g + j + m;
        if let Some(h) = digitize(sum - e - f - g) {
            if let Some(p) = digitize(sum - d - h - l) {
                for o in 0..10 {
                    if let Some(k) = digitize(sum - c - g - o) {
                        if let Some(i) = digitize(sum - j - k - l) {
                            if let Some(n) = digitize(sum - m - o - p) {
                                if let Some(b) = digitize(sum - f - j - n) {
                                    if let Some(a) = digitize(sum - b - c - d) {
                                        if a + f + k + p == sum && a + e + i + m == sum {
                                            ans += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    ans.to_string()
}
