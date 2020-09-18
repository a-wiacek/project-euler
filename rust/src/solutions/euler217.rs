use cached::proc_macro::cached;

// Input: How long number has to be,
//        what sum of digits must it have,
//        can we have leading zero?
// Output: How many numbers are there and what is their sum (mod)?
fn m(x: u128) -> u128 {
    x % 3u128.pow(15)
}

fn p(n: u32) -> u128 {
    m(10u128.pow(n))
}

#[cached]
fn count_nums(len: u32, sum_of_digits: u128, leading_zero_allowed: bool) -> (u128, u128) {
    if len == 1 {
        if sum_of_digits > 9 {
            (0, 0)
        } else if leading_zero_allowed {
            (1, sum_of_digits)
        } else {
            (if sum_of_digits > 0 { 1 } else { 0 }, sum_of_digits)
        }
    } else {
        (if leading_zero_allowed { 0 } else { 1 }..10)
            .filter_map(|d| {
                if sum_of_digits < d {
                    None
                } else {
                    let (how_many, total_sum) = count_nums(len - 1, sum_of_digits - d, true);
                    Some((how_many, total_sum + d * how_many * p(len - 1)))
                }
            })
            .fold((0, 0), |(a, b), (c, d)| (m(a + c), m(b + d)))
    }
}

fn balanced(len: u32) -> u128 {
    fn balanced_(len: u32, sum_of_digits: u128) -> u128 {
        let n = len / 2;
        let (upper_count, upper) = count_nums(n, sum_of_digits, false);
        let (lower_count, lower) = count_nums(n, sum_of_digits, true);
        if len % 2 == 0 {
            lower_count * upper * p(n) + upper_count * lower
        } else {
            10 * (lower_count * upper * p(n + 1) + upper_count * lower)
                + lower_count * upper_count * 45 * p(n)
        }
    }
    if len == 1 {
        45
    } else {
        (1..=9 * (len as u128 / 2))
            .map(|sum_of_digits| balanced_(len, sum_of_digits))
            .fold(0, |x, y| m(x + y))
    }
}

pub fn euler217() -> String {
    (1..=47).map(balanced).fold(0, |x, y| m(x + y)).to_string()
}
