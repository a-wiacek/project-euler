use num::{
    pow::{checked_pow, Pow},
    traits::CheckedMul,
    Integer, One,
};
use primal::Sieve;
use std::cmp::Ordering;
use std::default::Default;
use std::ops::Mul;

// This structure keeps information about prime factors building value.
// The invariant of the structure is that factors are prime,
// sorted in ascending order and multiply to value.
#[derive(Clone)]
pub struct Factorization<T> {
    factors: Vec<(T, usize)>, // (prime p, amount of p appearing in factorization of value)
    value: Option<T>,         // value of factorization (unless it is too big to fit here)
}

impl Factorization<usize> {
    pub fn new(value: usize, sieve: &Sieve) -> Factorization<usize> {
        Factorization {
            factors: sieve.factor(value).unwrap(),
            value: Some(value),
        }
    }
}

impl<T: One> Default for Factorization<T> {
    fn default() -> Self {
        Factorization {
            factors: vec![],
            value: Some(T::one()),
        }
    }
}

impl<T> Factorization<T>
where
    T: Integer + Clone + CheckedMul,
{
    pub fn new_unsafe(factors: Vec<(T, usize)>, value: Option<T>) -> Factorization<T> {
        Factorization { factors, value }
    }

    pub fn new_prime(p: T, times: usize) -> Factorization<T> {
        Factorization {
            factors: vec![(p.clone(), times)],
            value: checked_pow(p, times),
        }
    }

    pub fn factors(self) -> Vec<(T, usize)> {
        self.factors
    }

    pub fn value_safe(&self) -> Option<T> {
        self.value.clone()
    }

    pub fn value(&self) -> T {
        self.value_safe().unwrap()
    }

    pub fn try_div(self, rhs: &Self) -> Option<Self> {
        let value = match (&self.value, &rhs.value) {
            (Some(a), Some(b)) => {
                if a.is_multiple_of(b) {
                    Some(a.clone() / b.clone())
                } else {
                    return None;
                }
            }
            _ => None,
        };

        let mut factors = vec![];
        let mut self_it = self.factors.into_iter().peekable();
        let mut rhs_it = (&rhs.factors).iter().peekable();
        loop {
            match (self_it.peek(), rhs_it.peek()) {
                (None, None) => break,
                (Some(_), None) => {
                    for (p, times_p) in self_it {
                        factors.push((p, times_p));
                    }
                    break;
                }
                (None, Some(_)) => return None,
                (Some((p, _)), Some((q, _))) => match p.cmp(q) {
                    Ordering::Less => factors.push(self_it.next().unwrap()),
                    Ordering::Equal => {
                        let (p, times_p) = self_it.next().unwrap();
                        let (_, times_q) = rhs_it.next().unwrap();
                        match times_p.cmp(&times_q) {
                            Ordering::Less => return None,
                            Ordering::Equal => {}
                            Ordering::Greater => {
                                factors.push((p, times_p - *times_q));
                            }
                        }
                    }
                    Ordering::Greater => return None,
                },
            }
        }
        Some(Factorization { factors, value })
    }
}

impl<T> Mul for Factorization<T>
where
    T: Integer + Clone + CheckedMul,
{
    type Output = Factorization<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let value = match (&self.value, &rhs.value) {
            (Some(a), Some(b)) => a.checked_mul(b),
            _ => None,
        };
        let mut factors = vec![];
        let mut self_it = self.factors.into_iter().peekable();
        let mut rhs_it = rhs.factors.into_iter().peekable();
        loop {
            match (self_it.peek(), rhs_it.peek()) {
                (None, None) => break,
                (Some(_), None) => {
                    for (p, times_p) in self_it {
                        factors.push((p, times_p));
                    }
                    break;
                }
                (None, Some(_)) => {
                    for (q, times_q) in rhs_it {
                        factors.push((q, times_q));
                    }
                    break;
                }
                (Some((p, _)), Some((q, _))) => match p.cmp(q) {
                    Ordering::Less => factors.push(self_it.next().unwrap()),
                    Ordering::Equal => {
                        let (_, times_p) = self_it.next().unwrap();
                        let (p, times_q) = rhs_it.next().unwrap();
                        factors.push((p, times_p + times_q));
                    }
                    Ordering::Greater => factors.push(rhs_it.next().unwrap()),
                },
            }
        }
        Factorization { factors, value }
    }
}

impl<'a, T> Mul<&'a Factorization<T>> for Factorization<T>
where
    T: Integer + Clone + CheckedMul,
{
    type Output = Factorization<T>;

    fn mul(self, rhs: &Factorization<T>) -> Self::Output {
        let value = match (&self.value, &rhs.value) {
            (Some(a), Some(b)) => a.checked_mul(b),
            _ => None,
        };
        let mut factors = vec![];
        let mut self_it = self.factors.into_iter().peekable();
        let mut rhs_it = (&rhs.factors).iter().peekable();
        loop {
            match (self_it.peek(), rhs_it.peek()) {
                (None, None) => break,
                (Some(_), None) => {
                    for (p, times_p) in self_it {
                        factors.push((p, times_p));
                    }
                    break;
                }
                (None, Some(_)) => {
                    for (q, times_q) in rhs_it {
                        factors.push((q.clone(), *times_q));
                    }
                    break;
                }
                (Some((p, _)), Some((q, _))) => match p.cmp(q) {
                    Ordering::Less => factors.push(self_it.next().unwrap()),
                    Ordering::Equal => {
                        let (p, times_p) = self_it.next().unwrap();
                        let (_, times_q) = rhs_it.next().unwrap();
                        factors.push((p, times_p + *times_q));
                    }
                    Ordering::Greater => factors.push(rhs_it.next().cloned().unwrap()),
                },
            }
        }
        Factorization { factors, value }
    }
}

impl<'a, T> Mul for &'a Factorization<T>
where
    T: Integer + Clone + CheckedMul,
{
    type Output = Factorization<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let value = match (&self.value, &rhs.value) {
            (Some(a), Some(b)) => a.checked_mul(b),
            _ => None,
        };
        let mut factors = vec![];
        let mut self_it = (&self.factors).iter().peekable();
        let mut rhs_it = (&rhs.factors).iter().peekable();
        loop {
            match (self_it.peek(), rhs_it.peek()) {
                (None, None) => break,
                (Some(_), None) => {
                    for (p, times_p) in self_it {
                        factors.push((p.clone(), *times_p));
                    }
                    break;
                }
                (None, Some(_)) => {
                    for (q, times_q) in rhs_it {
                        factors.push((q.clone(), *times_q));
                    }
                    break;
                }
                (Some((p, _)), Some((q, _))) => match p.cmp(q) {
                    Ordering::Less => factors.push(self_it.next().cloned().unwrap()),
                    Ordering::Equal => {
                        let (p, times_p) = self_it.next().unwrap();
                        let (_, times_q) = rhs_it.next().unwrap();
                        factors.push((p.clone(), *times_p + *times_q));
                    }
                    Ordering::Greater => factors.push(rhs_it.next().cloned().unwrap()),
                },
            }
        }
        Factorization { factors, value }
    }
}

impl<T> Pow<usize> for Factorization<T>
where
    T: Integer + Clone + CheckedMul,
{
    type Output = Self;

    fn pow(self, exp: usize) -> Self::Output {
        if exp == 0 {
            Default::default()
        } else {
            let Factorization { factors, value } = self;
            Factorization {
                factors: factors
                    .into_iter()
                    .map(|(p, times)| (p, times * exp))
                    .collect(),
                value: value.and_then(|value| checked_pow(value, exp)),
            }
        }
    }
}

impl<'a, T> Pow<usize> for &'a Factorization<T>
where
    T: Integer + Clone + CheckedMul,
{
    type Output = Factorization<T>;

    fn pow(self, exp: usize) -> Self::Output {
        if exp == 0 {
            Default::default()
        } else {
            let Factorization { factors, value } = self;
            Factorization {
                factors: factors
                    .into_iter()
                    .map(|(p, times)| (p.clone(), times * exp))
                    .collect(),
                value: value.clone().and_then(|value| checked_pow(value, exp)),
            }
        }
    }
}
