module Solutions.Euler146 where
import Math.NumberTheory.Primes.Testing(isPrime)

check n =
    let f = map ((n * n)+)
    in all isPrime (f [1, 3, 7, 9, 13, 27]) && not (any isPrime $ f [11, 17, 19, 21, 23])
f n = sum $ filter check [10, 20..n - 1]

euler146 :: IO String
euler146 = return $ show $ f 150000000