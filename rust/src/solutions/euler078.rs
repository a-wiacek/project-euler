pub struct Pentagonal {
    k: i64,
    sign: i64,
    first: bool,
}

impl Pentagonal {
    pub fn new() -> Pentagonal {
        Pentagonal {
            k: 1,
            sign: 1,
            first: true,
        }
    }
}

impl Iterator for Pentagonal {
    type Item = (i64, i64);
    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            Some((self.sign, self.k * (3 * self.k - 1) / 2))
        } else {
            let ans = (self.sign, self.k * (3 * self.k + 1) / 2);
            self.k += 1;
            self.sign = -self.sign;
            self.first = true;
            Some(ans)
        }
    }
}

pub fn euler078() -> String {
    let bound: usize = 75000;
    let mut partitions = vec![1i64; bound];
    for n in 1..bound {
        partitions[n] = Pentagonal::new()
            .take_while(|&(_, k)| k <= n as i64)
            .map(|(sign, k)| sign * partitions[n - k as usize])
            .sum::<i64>()
            % 1000000;
    }
    (1usize..)
        .find(|&i| partitions[i] == 0)
        .unwrap()
        .to_string()
}
