module Solutions.Euler060 where
import Utils.NumberTheory(unPrime, primesArrayUpTo, primesUpTo)
import Data.Array.Unboxed

primes = primesArrayUpTo 99999999
primesList = map unPrime $ primesUpTo 9999 :: [Int]

concatPrime a b = primes ! read (show a ++ show b) && primes ! read (show b ++ show a)

compute = head [p1 + p2 + p3 + p4 + p5 |
    p1 <- primesList,
    p2 <- takeWhile (<p1) primesList,
    concatPrime p1 p2,
    p3 <- takeWhile (<p2) primesList,
    all (concatPrime p3) [p1, p2],
    p4 <- takeWhile (<p3) primesList,
    all (concatPrime p4) [p1, p2, p3],
    p5 <- takeWhile (<p4) primesList,
    all (concatPrime p5) [p1, p2, p3, p4]]

euler060 :: IO String
euler060 = return $ show compute