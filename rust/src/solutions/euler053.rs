pub fn euler053() -> String {
    let mut ans = 0;
    for x in 1..=100 {
        for y in 0..=x {
            if num::integer::binomial::<u128>(x, y) > 1000000 {
                ans += 1;
            }
        }
    }
    ans.to_string()
}
