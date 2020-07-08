fn day_in_month(month: i32, year: i32) -> i32 {
    if [0, 2, 4, 6, 7, 9, 11].contains(&month) {
        31
    } else if [3, 5, 8, 10].contains(&month) {
        30
    } else if year % 4 == 0 {
        29
    } else {
        28
    }
}

struct Date {
    weekday: i32,
    month: i32,
    year: i32,
}

impl Date {
    fn step(&mut self) {
        self.weekday = (self.weekday + day_in_month(self.month, self.year)) % 7;
        if self.month == 11 {
            self.month = 0;
            self.year += 1;
        } else {
            self.month += 1;
        }
    }
}

fn query(date: &mut Date) -> i32 {
    let mut ans = 0;
    while date.year < 2001 {
        if date.weekday == 6 {
            ans += 1;
        }
        date.step()
    }
    ans
}

pub fn euler019() -> String {
    query(&mut Date {
        weekday: 1,
        month: 0,
        year: 1901,
    })
    .to_string()
}
