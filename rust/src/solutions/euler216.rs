pub fn euler216() -> String {
    let bound = 50_000_000;
    let mut ans = 0;
    let mut vec = Vec::<usize>::with_capacity(bound + 1);
    vec.push(0);
    for i in 1..=bound {
        vec.push(2 * i * i - 1);
    }
    let uberdiv = |r: &mut usize, p| {
        while *r % p == 0 {
            *r /= p
        }
    };
    for i in 2..=bound {
        let p = vec[i];
        if p > 1 {
            if p == 2 * i * i - 1 {
                ans += 1;
            }
            for j in (p + i..=bound).step_by(p) {
                uberdiv(vec.get_mut(j).unwrap(), p);
            }
            let k = std::cmp::max(1, 2 * (i + 2) / p);
            for j in (k * p - i..=bound).step_by(p) {
                uberdiv(vec.get_mut(j).unwrap(), p);
            }
        }
    }
    ans.to_string()
}
