use itertools::Itertools;

fn scores(times: usize, max: usize) -> impl Iterator<Item = (usize, usize)> {
    let mut count = vec![0; times * max + 1];
    for score in (0..times)
        .map(|_| 1..=max)
        .multi_cartesian_product()
        .map(|vec| vec.into_iter().sum::<usize>())
    {
        count[score] += 1;
    }
    (0..).zip(count.into_iter())
}

pub fn euler205() -> String {
    let mut wins = 0;
    for (pscore, ptimes) in scores(9, 4) {
        for (cscore, ctimes) in scores(6, 6) {
            if pscore > cscore {
                wins += ptimes * ctimes;
            }
        }
    }
    format!("{:.7}", wins as f64 / (4u64.pow(9) * 6u64.pow(6)) as f64)
}
