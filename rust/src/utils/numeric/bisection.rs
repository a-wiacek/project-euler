use num::{One, Signed, Zero};
use std::ops::{Add, Div};

// Perform iterations of bisection method. It is assumed that begin < end
// and f(begin).signum() != f(end).signum().
pub fn bisection<F, A, B>(f: F, mut begin: A, mut end: A, iterations: usize) -> A
where
    F: Fn(A) -> B,
    A: Add<Output = A> + Div<Output = A> + One + Clone,
    B: Signed + Zero,
{
    let mut f_begin = f(begin.clone());
    for _ in 0..iterations {
        let mid = (begin.clone() + end.clone()) / (A::one() + A::one());
        let f_mid = f(mid.clone());
        if f_begin.signum() == f_mid.signum() {
            begin = mid;
            f_begin = f_mid;
        } else {
            end = mid;
        }
    }
    (begin.clone() + end.clone()) / (A::one() + A::one())
}
