module Solutions.Euler131 where
import Utils.NumberTheory(unPrime, primes)

common [] _ = 0
common _ [] = 0
common (a : b) (c : d) = case compare a c of
    LT -> common b (c : d)
    EQ -> 1 + common b d
    GT -> common (a : b) d

cubediffs = c 1 where c n = ((n + 1)^3 - n^3) : c (n + 1)

f = takeWhile (<=1000000)

euler131 :: IO String
euler131 = return $ show $ common (f $ map unPrime primes) (f cubediffs)