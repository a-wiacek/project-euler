use crate::utils::numeric::digits::Digits;

#[derive(Eq, PartialEq)]
enum Segment {
    U,
    LU,
    RU,
    M,
    LD,
    RD,
    D,
}

fn d(n: usize) -> u8 {
    (n % 10) as u8
}

struct Number(Vec<Segment>);
fn segments(n: u8) -> Number {
    use Segment::*;
    Number(match n {
        0 => vec![U, LU, RU, LD, RD, D],
        1 => vec![RU, RD],
        2 => vec![U, RU, M, LD, D],
        3 => vec![U, RU, M, RD, D],
        4 => vec![LU, RU, M, RD],
        5 => vec![U, LU, M, RD, D],
        6 => vec![U, LU, M, LD, RD, D],
        7 => vec![U, LU, RU, RD],
        8 => vec![U, LU, RU, M, LD, RD, D],
        9 => vec![U, LU, RU, M, RD, D],
        _ => panic!(),
    })
}

fn cost(n: u8) -> usize {
    segments(n).0.len()
}

fn total_cost(mut n: usize) -> usize {
    let mut ans = 0;
    while n > 0 {
        ans += cost(d(n));
        n /= 10;
    }
    ans
}

fn cost_diff(m: u8, n: u8) -> usize {
    let m = segments(m);
    let n = segments(n);
    let mut ans = 0;
    for seg in segments(8).0 {
        if m.0.contains(&seg) as u8 + n.0.contains(&seg) as u8 == 1 {
            ans += 1;
        }
    }
    ans
}

fn total_cost_diff(mut m: usize, mut n: usize) -> usize {
    let mut ans = 0;
    loop {
        if m > 0 && n > 0 {
            ans += cost_diff(d(m), d(n));
            m /= 10;
            n /= 10;
        } else {
            break ans + total_cost(m) + total_cost(n);
        }
    }
}

fn energy_calc<F>(transition_cost: F, numbers: &Vec<usize>) -> usize
where
    F: Fn(usize, usize) -> usize,
{
    total_cost(*numbers.first().unwrap())
        + total_cost(*numbers.last().unwrap())
        + numbers
            .windows(2)
            .map(|window| match window {
                &[m, n] => transition_cost(m, n),
                _ => panic!(),
            })
            .sum::<usize>()
}

fn digital_root_steps(mut n: usize) -> Vec<usize> {
    let mut ans = vec![n];
    while n >= 10 {
        n = n.digits().into_iter().sum();
        ans.push(n);
    }
    ans
}

pub fn euler315() -> String {
    let lower_bound = 10000000;
    let upper_bound = 20000000;
    let nums = primal::Sieve::new(upper_bound)
        .primes_from(lower_bound)
        .take_while(|&p| p < upper_bound)
        .map(digital_root_steps)
        .collect::<Vec<_>>();
    let sam_cost = nums
        .iter()
        .map(|x| energy_calc(|m, n| total_cost(m) + total_cost(n), x))
        .sum::<usize>();
    let max_cost = nums
        .iter()
        .map(|x| energy_calc(total_cost_diff, x))
        .sum::<usize>();
    (sam_cost - max_cost).to_string()
}
