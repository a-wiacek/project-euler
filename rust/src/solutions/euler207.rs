/*
Let x = 2^t, then we solve equation x^2 = x + k -> x^2 - x - k = 0
We are interested only in positive solution, so x = (1 + sqrt (1 + 4k)) / 2,
so k = ((2x - 1)^2 - 1) / 4. If x is integer, k is also integer,
// since square of odd number is equal to 1 modulo 4.
*/

pub fn euler207() -> String {
    let mut perfect_p: u64 = 1;
    let mut total_p: u64 = 2;
    while 12345 * perfect_p >= total_p {
        total_p += 1;
        if total_p.next_power_of_two() == total_p {
            perfect_p += 1;
        }
    }
    (((2 * total_p + 1).pow(2) - 1) / 4).to_string()
}
