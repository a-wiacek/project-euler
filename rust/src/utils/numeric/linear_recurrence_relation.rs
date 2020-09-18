use nalgebra::*;
use num::{integer::Integer, One, Zero};

// Structure describing linear recurrence relations (LRR), i. e.:
// x_0 = a_0, ..., x_{k - 1} = a_{k - 1},
// x_n = c_0 x_{n - 1} + ... + c_{k - 1} x_{n - k} for n >= k

pub fn matrix_pow<T, U>(mut base: DMatrix<T>, mut exp: U) -> DMatrix<T>
where
    T: Scalar + Zero + One + ClosedAdd + ClosedMul,
    U: Integer,
{
    let n = base.ncols();
    let mut acc = DMatrix::<T>::identity(n, n);
    while exp > U::one() {
        if exp.is_odd() {
            acc = &acc * &base;
        }
        exp = exp / (U::one() + U::one());
        base = &base * &base;
    }
    if exp == U::one() {
        acc = acc * base;
    }
    acc
}

pub fn matrix_pow_mod<T, U>(mut base: DMatrix<T>, mut exp: U, p: T) -> DMatrix<T>
where
    T: Scalar + Zero + One + ClosedAdd + ClosedMul + std::cmp::PartialOrd,
    for<'a> &'a T: std::ops::Rem<Output = T>,
    U: Integer,
{
    let n = base.ncols();
    let mut acc = DMatrix::<T>::identity(n, n);
    let f = |x| {
        let y = &x % &p;
        if y < T::zero() {
            y + p.clone()
        } else {
            y
        }
    };
    while exp > U::one() {
        if exp.is_odd() {
            acc = &acc * &base;
            acc = acc.map(f);
        }
        exp = exp / (U::one() + U::one());
        base = &base * &base;
        base = base.map(f);
    }
    if exp == U::one() {
        acc = acc * base;
        acc = acc.map(f);
    }
    acc
}

pub struct LRR<T>
where
    T: Scalar + One + Zero + ClosedAdd + ClosedMul + std::cmp::PartialOrd,
    for<'a> &'a T: std::ops::Rem<Output = T>,
{
    recurrence_matrix: DMatrix<T>,
    initial_values: DVector<T>,
}

impl<T> LRR<T>
where
    T: Scalar + One + Zero + ClosedAdd + ClosedMul + std::cmp::PartialOrd,
    for<'a> &'a T: std::ops::Rem<Output = T>,
{
    // Construct LRR from two vectors: [a_0, ..., a_{k - 1}] and [c_0, ..., c_{k - 1}].
    // The structure requires that those vectors have the same length,
    // so `new` will panic if they are not of the same length.
    pub fn new(initial_values: Vec<T>, mut equation_coefficients: Vec<T>) -> LRR<T> {
        let n = initial_values.len();
        if n != equation_coefficients.len() {
            panic!(
                "Tried to initialize linear reccurence relation with vectors of length {} != {}",
                n,
                equation_coefficients.len()
            );
        }
        equation_coefficients.reverse();
        LRR {
            recurrence_matrix: DMatrix::<T>::from_fn(n, n, |row, col| {
                if row == n - 1 {
                    equation_coefficients[col].clone()
                } else if let Some(1) = col.checked_sub(row) {
                    T::one()
                } else {
                    T::zero()
                }
            }),
            initial_values: DVector::<T>::from_vec(initial_values),
        }
    }

    // Return a_n. Care for overflows!
    #[allow(dead_code)]
    pub fn at<U: Integer>(&self, n: U) -> T {
        let mp = matrix_pow(self.recurrence_matrix.clone(), n);
        (&mp * &self.initial_values)[0].clone()
    }

    // Return a_n mod p.
    pub fn at_mod<U: Integer>(&self, n: U, p: T) -> T {
        let mp = matrix_pow_mod(self.recurrence_matrix.clone(), n, p.clone());
        let y = &(&mp * &self.initial_values)[0] % &p;
        if y < T::zero() {
            y + p
        } else {
            y
        }
    }
}
