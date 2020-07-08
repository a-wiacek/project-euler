fn laminae(mut l: i64, mut t: i64) -> i64 {
    let mut ans = 0;
    loop {
        if 4 * l - 4 > 1000000 {
            break ans;
        } else if 4 * t * (l - t) > 1000000 || 2 * t >= l {
            l += 1;
            t = 1;
        } else {
            ans += 1;
            t += 1;
        } 
    }
}

pub fn euler173() -> String {
    laminae(3, 1).to_string()
}
