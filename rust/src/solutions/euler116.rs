fn generator(k: usize) -> Vec<u64> {
    let mut vec = vec![1u64; 51];
    for i in k..51 {
        vec[i] = vec[i - 1] + vec[i - k];
    }
    for i in 0..51 {
        vec[i] -= 1;
    }
    vec
}

pub fn euler116() -> String {
    (generator(2)[50] + generator(3)[50] + generator(4)[50]).to_string()
}
