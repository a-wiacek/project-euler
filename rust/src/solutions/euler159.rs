use crate::utils::numeric::digits::Digits;

pub fn euler159() -> String {
    let mut mdrs_arr = Vec::new();
    for n in 2usize..1000000 {
        let mdrs_n = (2..)
            .take_while(|&x| x * x <= n)
            .filter(|&x| n % x == 0)
            .fold(n.digital_root(), |s, x| {
                std::cmp::max(s, mdrs_arr[x - 2] + mdrs_arr[n / x - 2])
            });
        mdrs_arr.push(mdrs_n);
    }
    mdrs_arr.into_iter().sum::<usize>().to_string()
}
