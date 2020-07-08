use crate::utils::number_theory::totient::totient_array;

fn compute(n: usize) -> usize {
    totient_array::<u32>(n)
        .into_iter()
        .step_by(2)
        .map(|i| i as usize)
        .sum()
}

pub fn euler512() -> String {
    compute(500000000).to_string()
}
