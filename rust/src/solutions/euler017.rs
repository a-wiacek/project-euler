fn itos(number: usize) -> String {
    let first = [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let dozens = [
        "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];
    match number {
        1000 => String::from("onethousand"),
        n if n < 20 => String::from(first[n - 1]),
        n if n >= 100 => {
            let prefix = String::from(first[n / 100 - 1]) + "hundred";
            let rem = n % 100;
            if rem == 0 {
                prefix
            } else {
                prefix + "and" + &itos(rem)
            }
        }
        _ => {
            let prefix = String::from(dozens[number / 10 - 2]);
            let rem = number % 10;
            if rem == 0 {
                prefix
            } else {
                prefix + first[rem - 1]
            }
        }
    }
}

pub fn euler017() -> String {
    (1..=1000).map(|n| itos(n).len()).sum::<usize>().to_string()
}
