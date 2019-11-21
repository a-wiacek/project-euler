module Solutions.Euler087 where
import Utils.List(uniques)
import Utils.NumberTheory(unPrime, primes)

ps = map unPrime primes :: [Int]

compute = length $ uniques [p2^2 + p3^3 + p4^4 |
    p4 <- takeWhile (\p -> p^4 < 50000000) ps,
    p3 <- takeWhile (\p -> p4^4 + p^3 < 50000000) ps,
    p2 <- takeWhile (\p -> p4^4 + p3^3 + p^2 < 50000000) ps]

euler087 :: IO String
euler087 = return $ show compute