use crate::utils::numeric::linear_recurrence_relation::LRR;

pub fn euler237() -> String {
    LRR::<i64>::new(vec![0, 1, 1, 4], vec![2, 2, -2, 1])
        .at_mod(10usize.pow(12), 10i64.pow(8))
        .to_string()
}
