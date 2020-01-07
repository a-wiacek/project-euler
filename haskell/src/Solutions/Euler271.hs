module Solutions.Euler271 where
import Utils.NumberTheory(unPrime, primesUpTo)
import Utils.Numeric(fastPowerMod)

{-
Factorizing 13082761331670030, we can see that it is product of all primes up to 43.
By chinese remainder theorem, this congruence is equal to following congruences:
 x^3 = 1 mod 2
 x^3 = 1 mod 3
 ...
 x^3 = 1 mod 43
-}
primes = map unPrime $ primesUpTo 43 :: [Integer]
q = product primes

smallSolutions p = [x | x <- [1..p - 1], x^3 `mod` p == 1]
multiplier p = let m = q `div` p in m * fastPowerMod m (p - 2) p
inverse = map multiplier primes
compute = pred $ sum
        $ map (\solChoice -> sum (zipWith (*) solChoice inverse) `mod` q)
        $ mapM smallSolutions primes

euler271 :: IO String
euler271 = return $ show compute 