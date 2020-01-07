pub fn euler002() -> String {
    let mut sum = 0;
    let mut low = 1;
    let mut high = 1;
    while high < 4000000 {
        let tmp = low + high;
        low = high;
        high = tmp;
        if high % 2 == 0 {
            sum += high;
        }
    }
    sum.to_string()
}