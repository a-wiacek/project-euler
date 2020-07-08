use cached::proc_macro::cached;

#[cached]
fn f(n: u128) -> u128 {
    if n <= 1 {
        1
    } else {
        let n2 = n / 2;
        let fn2 = f(n2);
        if n % 2 == 0 {
            fn2
        } else {
            fn2 + f(n2 + 1)
        }
    }
}

pub fn euler169() -> String {
    f(10u128.pow(25) + 1).to_string()
}
