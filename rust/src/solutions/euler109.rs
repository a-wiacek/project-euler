use itertools::Itertools;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Dart {
    Single(u32),
    Double(u32),
    Triple(u32),
}

impl Dart {
    fn value(&self) -> u32 {
        match self {
            Dart::Single(d) => *d,
            Dart::Double(d) => 2 * *d,
            Dart::Triple(d) => 3 * *d,
        }
    }
}

fn all_darts() -> Vec<Dart> {
    let mut all_darts = vec![Dart::Single(25), Dart::Double(25)];
    for d in 1..=20 {
        all_darts.extend(vec![Dart::Single(d), Dart::Double(d), Dart::Triple(d)]);
    }
    all_darts
}

fn check_out_up_to(n: u32) -> usize {
    (1..=3)
        .flat_map(|i| {
            (0..i)
                .map(|_| all_darts())
                .multi_cartesian_product()
                .filter(move |throws| {
                    (i < 3 || throws[0] >= throws[1])
                        && throws.iter().map(|throw| throw.value()).sum::<u32>() <= n
                        && throws.last().map_or(false, |throw| match throw {
                            Dart::Double(_) => true,
                            _ => false,
                        })
                })
        })
        .count()
}

pub fn euler109() -> String {
    check_out_up_to(99).to_string()
}
