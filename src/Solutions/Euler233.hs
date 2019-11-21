module Solutions.Euler233 where
import Utils.NumberTheory(primes, unPrime)
import Data.List(partition, sort)

primes1, primes1' :: [Int]
bound = 10^11 :: Int
(primes1, primes1') = partition (\p -> p `mod` 4 == 1) $ map unPrime primes

qmin :: Int
qmin = 5^3 * 13^2 * 17
qproduct :: [Int]
qproduct = sort $ qpr (bound `div` qmin) primes1' where
    qpr :: Int -> [Int] -> [Int]
    qpr max (p:ps) = if p > max
        then [1]
        else map (*p) (qpr (max `div` p) (p:ps)) ++ qpr max ps

case1 :: [Int]
case1 = [p * q |
    p1 <- takeWhile (\p -> p^7 <= bound) primes1,
    p2 <- takeWhile (\p -> p1^7 * p^3 <= bound) primes1,
    p1 /= p2,
    let p = p1^7 * p2^3,
    q <- takeWhile (\qs -> qs * p <= bound) qproduct]

case2 :: [Int]
case2 = [p * q |
    p1 <- takeWhile (\p -> p^10 <= bound) primes1,
    p2 <- takeWhile (\p -> p1^10 * p^2 <= bound) primes1,
    p1 /= p2,
    let p = p1^10 * p2^2,
    q <- takeWhile (\qs -> qs * p <= bound) qproduct]

case3 :: [Int]
case3 = [p * q |
    p1 <- takeWhile (\p -> p^3 <= bound) primes1,
    p2 <- takeWhile (\p -> p1^3 * p^2 <= bound) primes1,
    p1 /= p2,
    p3 <- takeWhile (\p -> p1^3 * p2^2 * p <= bound) primes1,
    p1 /= p3,
    p2 /= p3,
    let p = p1^3 * p2^2 * p3,
    q <- takeWhile (\qs -> qs * p <= bound) qproduct]

euler233 :: IO String
euler233 = return $ show $ sum $ map sum [case1, case2, case3]