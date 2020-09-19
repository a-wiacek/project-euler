use num::{integer::Roots, FromPrimitive, Integer};

// Function produces array of size n
// a[i] contains value of phi(i + 1)
pub fn totient_array<T>(n: usize) -> Vec<T>
where
    T: Integer + Clone + FromPrimitive,
{
    let mut arr: Vec<T> = (1..=n).map(|x| T::from_usize(x).unwrap()).collect();
    for i in 1..n {
        let ii = T::from_usize(i + 1).unwrap();
        if arr[i] == ii {
            arr[i] = arr[i].clone() - T::one();
            for j in (i + i + 1..n).step_by(i + 1) {
                arr[j] = (arr[j].clone() / ii.clone()) * (ii.clone() - T::one());
            }
        }
    }
    arr
}

// Implementation based on
// https://hackage.haskell.org/package/arithmoi-0.11.0.1/docs/src/Math.NumberTheory.MoebiusInversion.html

pub fn totient_sum<T>(n: usize) -> T
where
    T: Integer + Clone + Roots + FromPrimitive,
{
    match n {
        0 => return T::zero(),
        1 => return T::one(),
        2 => return T::one() + T::one(),
        3 => return T::from_usize(4).unwrap(),
        _ => {}
    }

    let fun = |k: usize| {
        let kk = T::from_usize(k).unwrap();
        kk.clone() * (kk + T::one()) / (T::one() + T::one())
    };

    let k0 = (n / 2).sqrt();
    let mk0 = n / (k0 + k0 + 1);
    let kmax = |a, m| (a / m - 1) / 2;

    let mut small = vec![T::zero(); mk0 + 1];
    small[1] = fun(1);
    if mk0 >= 2 {
        small[2] = fun(2) - fun(1);
    }

    let mut switch = 1;
    let mut change = 8;
    let mut i = 3;

    loop {
        // calcit
        if mk0 < i {
            break;
        } else if i == change {
            change = change + 4 * switch + 6;
            switch += 1;
        } else {
            let mut mloop = true;
            let mut acc = fun(i) - fun(i / 2);
            let mut k = (i - 1) / 2;
            let mut m = 1;
            loop {
                // mloop or kloop
                if mloop {
                    if k < switch {
                        mloop = false;
                        continue;
                    } else {
                        let val = small[m].clone();
                        m += 1;
                        let next_k = kmax(i, m);
                        acc = acc - T::from_usize(k - next_k).unwrap() * val;
                        k = next_k;
                    }
                } else {
                    // kloop
                    if k == 0 {
                        small[i] = acc;
                        i += 1;
                        break;
                    } else {
                        let val = small[i / (k + k + 1)].clone();
                        acc = acc - val;
                        k -= 1;
                    }
                }
            }
        }
    }

    let mut large = vec![T::zero(); k0];

    let mut j = k0;
    loop {
        if j == 0 {
            break;
        } else if (2 * j - 1) * change <= n {
            change = change + 4 * switch + 6;
            switch += 1;
        } else {
            let i = n / (2 * j - 1);
            let mut mloop = true;
            let mut acc = fun(i) - fun(i / 2);
            let mut k = (i - 1) / 2;
            let mut m = 1;
            loop {
                // mloop or kloop
                if mloop {
                    if k < switch {
                        mloop = false;
                        continue;
                    } else {
                        let val = small[m].clone();
                        m += 1;
                        let next_k = kmax(i, m);
                        acc = acc - T::from_usize(k - next_k).unwrap() * val;
                        k = next_k;
                    }
                } else {
                    // kloop
                    if k == 0 {
                        j -= 1;
                        large[j] = acc;
                        break;
                    } else {
                        let m = i / (k + k + 1);
                        let val = if m <= mk0 {
                            small[m].clone()
                        } else {
                            large[k * (2 * j - 1) + j - 1].clone()
                        };
                        acc = acc - val;
                        k -= 1;
                    }
                }
            }
        }
    }

    large[0].clone()
}
