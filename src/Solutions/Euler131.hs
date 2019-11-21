module Solutions.Euler131 where
import Utils.NumberTheory(unPrime, primes)

common [] _ = 0
common _ [] = 0
common (a:b) (c:d)
    | a == c = 1 + common b d
    | a < c = common b (c:d)
    | otherwise = common (a:b) d

cubediffs = c 1 where c n = ((n + 1)^3 - n^3) : c (n + 1)

f = takeWhile (<=1000000)

euler131 :: IO String
euler131 = return $ show $ common (f $ map unPrime primes) (f cubediffs)