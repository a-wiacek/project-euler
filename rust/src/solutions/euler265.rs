use std::iter::{empty, once};

fn search(n: u32, mut l: Vec<u32>) -> Box<dyn Iterator<Item = Vec<u32>>> {
    let npow = 2usize.pow(n);
    if l.len() == npow {
        Box::new(once(l))
    } else {
        let last = *l.last().unwrap();
        let x = 2 * last % npow as u32;
        let i1: Box<dyn Iterator<Item = Vec<u32>>> = if l.contains(&x) {
            Box::new(empty())
        } else {
            let mut l2 = l.clone();
            l2.push(x);
            Box::new(search(n, l2))
        };
        let x = x + 1;
        let i2: Box<dyn Iterator<Item = Vec<u32>>> = if l.contains(&x) {
            Box::new(empty())
        } else {
            l.push(x);
            Box::new(search(n, l))
        };
        Box::new(i1.chain(i2))
    }
}

fn de_brujin_sequences(n: u32) -> impl Iterator<Item = Vec<bool>> {
    search(n, vec![0]).map(move |v| {
        let mut ans = vec![false; n as usize - 1];
        ans.extend(
            v.into_iter()
                .map(|d| (d & 1) != 0)
                .take(2usize.pow(n) - n as usize + 1),
        );
        ans
    })
}

pub fn euler265() -> String {
    de_brujin_sequences(5)
        .map(|v| v.into_iter().fold(0u64, |acc, b| 2 * acc + b as u64))
        .sum::<u64>()
        .to_string()
}
