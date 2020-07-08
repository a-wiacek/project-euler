use crate::utils::numeric::digits::{undigits, Digits};
use permutohedron::LexicalPermutation;

pub fn euler068() -> String {
    let mut v: Vec<u64> = (1..=10).collect();
    let mut ans: u64 = 0;
    loop {
        if let [l0, l1, l2, l3, l4, l5, l6, l7, l8, l9] = v[..] {
            let w1 = (l0, [l0, l1, l3]);
            let w2 = (l2, [l2, l3, l5]);
            let w3 = (l4, [l4, l5, l7]);
            let w4 = (l6, [l6, l7, l9]);
            let w5 = (l8, [l8, l9, l1]);
            let s = l0 + l1 + l3;
            if [w2, w3, w4, w5]
                .iter()
                .all(|w| w.1.iter().sum::<u64>() == s)
            {
                let mut ws = vec![w1, w2, w3, w4, w5];
                let wmin = ws.iter().min_by_key(|w| w.0).unwrap();
                let wmin_pos = ws.iter().position(|w| w == wmin).unwrap();
                ws.rotate_left(wmin_pos);
                let repr: Vec<u64> = ws
                    .iter()
                    .flat_map(|(_, v)| v)
                    .flat_map(|v| v.digits())
                    .collect();
                if repr.len() == 16 {
                    ans = std::cmp::max(ans, undigits(&repr));
                }
            }
        }
        if !v.next_permutation() {
            break ans.to_string();
        }
    }
}
