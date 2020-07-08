use cached::proc_macro::cached;

fn psum(a: i64, b: i64) -> i64 {
    (a + b) % 1000000007
}

#[cached]
fn exact_lambda_count(len: i64, vars: i64) -> i64 {
    if len <= 0 {
        0
    } else if len == 1 {
        vars
    } else {
        let application = (1..len - 2)
            .map(|m| exact_lambda_count(m, vars) * exact_lambda_count(len - 2 - m, vars))
            .fold(0, psum);
        let abstraction = exact_lambda_count(len - 5, vars + 1);
        psum(application, abstraction)
    }
}

fn lambda_count(max_len: i64) -> i64 {
    (6..=max_len)
        .map(|len| exact_lambda_count(len, 0))
        .fold(0, psum)
}

pub fn euler623() -> String {
    lambda_count(2000).to_string()
}
