module Solutions.Euler278 where
import Utils.NumberTheory(primesUpTo, unPrime)

f p q r = 2 * p * q * r - p * q - q * r - r * p

compute :: Int
compute = sum [f p q r | let primes = map unPrime (primesUpTo 5000),
                         p <- primes,
                         q <- primes,
                         p < q,
                         r <- primes,
                         q < r]

euler278 :: IO String
euler278 = return $ show compute