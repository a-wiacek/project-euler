use num::Integer;

// Compute (base ** exp) modulo p using fast exponentiation.
// It is assumed that exp >= 0 and p > 0.

pub trait ModPow {
    fn modpow(&self, exp: &Self, p: &Self) -> Self;
}

impl<T> ModPow for T
where
    T: Integer + Clone,
{
    fn modpow(&self, exp: &Self, p: &Self) -> Self {
        if exp.is_zero() {
            return T::one();
        }

        let mut exp = exp.clone();
        let mut ans = T::one();
        let mut base = self.clone() % p.clone();

        loop {
            if exp.is_odd() {
                ans = ans * base.clone() % p.clone();
            }
            if exp.is_one() {
                break ans;
            }
            exp = exp / (T::one() + T::one());
            base = base.clone() * base % p.clone();
        }
    }
}
