use itertools::Itertools;
use primal::Sieve;

// Details of algorithm: http://vixra.org/pdf/1310.0211v1.pdf
// Algorithm in pdf is counting triples one by one. Since answer is very close
// to 10^15, we can't count them one by one. We need to do it faster.
// https://math.stackexchange.com/questions/218890/how-many-numbers-in-a-given-range-are-coprime-to-n
// http://emis.impa.br/EMIS/journals/INTEGERS/papers/i41/i41.pdf (theorem 3b for k = 1)

fn power_mobius<'a>(numbers: &'a [usize]) -> Box<dyn Iterator<Item = i64> + 'a> {
    if let Some((&first, next)) = numbers.split_first() {
        let (then_1, then_2) = power_mobius(next).tee();
        Box::new(then_1.map(move |x| -x * first as i64).chain(then_2))
    } else {
        Box::new(std::iter::once(1))
    }
}

fn coprime_count(n: usize, b: usize, sieve: &Sieve) -> usize {
    let primes = sieve
        .factor(n)
        .unwrap()
        .into_iter()
        .map(|fact| fact.0)
        .collect_vec();
    power_mobius(&primes).map(|x| b as i64 / x).sum::<i64>() as usize
}

pub fn euler540() -> String {
    let n = 3141592653589793f64;
    let sieve = &Sieve::new((n * 0.6).sqrt() as usize);
    let mut ans = 0;

    // Case 1
    for ii in (1..=(n / (2.0 + 2f64.sqrt())).sqrt() as usize).step_by(2) {
        let i = ii as f64;
        let n1 = (i / 2f64.sqrt()) as usize;
        let nv = (((n + n - i * i).sqrt() - i) / 2.0) as usize;
        ans += coprime_count(ii, nv, sieve) - coprime_count(ii, n1, sieve);
    }

    // Case 2 (ii + ii trick allows us to exclude all even numbers)
    for ii in 1..=(n / (4.0 + 8f64.sqrt())).sqrt() as usize {
        let i = ii as f64;
        let n1 = (i * 2f64.sqrt()) as usize;
        let nv = ((n - i * i).sqrt() - i) as usize;
        ans += coprime_count(ii + ii, nv, sieve) - coprime_count(ii + ii, n1, sieve);
    }

    ans.to_string()
}
