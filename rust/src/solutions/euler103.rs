use itertools::Itertools;
use std::cmp::Ordering;

fn two_subsets<'a>(set: &'a [u32]) -> Box<dyn Iterator<Item = (Vec<u32>, Vec<u32>)> + 'a> {
    fn f<'a>(set: &'a [u32]) -> Box<dyn Iterator<Item = (Vec<u32>, Vec<u32>)> + 'a> {
        if set.is_empty() {
            Box::new(vec![(vec![], vec![])].into_iter())
        } else {
            Box::new(f(&set[1..]).flat_map(move |(left, right)| {
                let mut lefth = left.clone();
                lefth.push(set[0]);
                let mut righth = right.clone();
                righth.push(set[0]);
                vec![
                    (left.clone(), right.clone()),
                    (lefth, right),
                    (left, righth),
                ]
            }))
        }
    }
    Box::new(f(set).filter(|(left, right)| !left.is_empty() && !right.is_empty()))
}

pub fn test_set(set: &[u32]) -> bool {
    two_subsets(set).all(|(left, right)| {
        let cmp = left.len().cmp(&right.len());
        let left_sum = left.into_iter().sum::<u32>();
        let right_sum = right.into_iter().sum::<u32>();
        match cmp {
            Ordering::Less => left_sum < right_sum,
            Ordering::Equal => left_sum != right_sum,
            Ordering::Greater => left_sum > right_sum,
        }
    })
}

pub fn euler103() -> String {
    (20u32..46)
        .combinations(7)
        .filter(|set| test_set(&set[..]))
        .min_by_key(|v| v.iter().sum::<u32>())
        .unwrap()
        .into_iter()
        .map(|el| el.to_string())
        .collect()
}
