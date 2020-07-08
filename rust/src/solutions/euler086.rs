use num::integer::Roots;

fn computations(m: usize, total: usize) -> usize {
    if total > 1000000 {
        m - 1
    } else {
        let exact = (1..=m)
            .flat_map(|b| (1..=b).map(move |c| (m, b, c)))
            .filter(|&(a, b, c)| {
                let dist2 = a.pow(2) + (b + c).pow(2);
                dist2 == dist2.sqrt().pow(2)
            })
            .count();
        computations(m + 1, total + exact)
    }
}

pub fn euler086() -> String {
    computations(1, 0).to_string()
}
