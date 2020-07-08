use itertools::Itertools;

fn cubes(x: u64, y: u64, z: u64, n: u64) -> u64 {
    2 * (x * y + y * z + z * x) + 4 * (x + y + z + n - 2) * (n - 1)
}

pub fn euler126() -> String {
    let arg = 1000;
    let bound = 25000;
    let mut all_values = Vec::new();
    for x in (1..).take_while(|&t| cubes(t, t, t, 1) < bound) {
        for y in (x..).take_while(|&t| cubes(x, t, t, 1) < bound) {
            for z in (y..).take_while(|&t| cubes(x, y, t, 1) < bound) {
                for n in (1..).take_while(|&t| cubes(x, y, z, t) < bound) {
                    all_values.push(cubes(x, y, z, n));
                }
            }
        }
    }
    all_values.sort();
    all_values
        .into_iter()
        .group_by(|&x| x)
        .into_iter()
        .map(|(k, v)| (k, v.count()))
        .find(|&(_, n)| n == arg)
        .unwrap()
        .0
        .to_string()
}
