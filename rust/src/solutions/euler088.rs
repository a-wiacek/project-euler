fn check(rem_len: i64, rem_prod: i64, rem_sum: i64, max_n: i64) -> bool {
    if rem_len == 0 {
        rem_prod == 1 && rem_sum == 0
    } else {
        match rem_prod {
            1 => rem_sum == rem_len,
            _ if rem_prod < 1 => false,
            _ => (2..=max_n)
                .filter(|&n| rem_prod % n == 0)
                .any(|n| check(rem_len - 1, rem_prod / n, rem_sum - n, n)),
        }
    }
}

fn iter(n: i64, k: i64) -> i64 {
    if check(k, n, n, k) {
        n
    } else {
        iter(n + 1, k)
    }
}

fn product_sum(k: i64) -> i64 {
    iter(k, k)
}

pub fn euler088() -> String {
    let mut psums = (2..=12000).map(product_sum).collect::<Vec<i64>>();
    psums.sort();
    psums.dedup();
    psums.iter().sum::<i64>().to_string()
}
