module Solutions.Euler668 where
import Utils.NumberTheory(unPrime, primesUpTo)

bound = 10^5
primes = map unPrime $ primesUpTo bound
-- Let n = p_0 ... p_k, where p_0 >= p_1 >= ... and n <= 10^10.
-- Let P = p_1 ... p_k
-- We have two cases:
-- 1) p_0 >= 10^5 -> n is not smooth (suppose it is, then p_0 < P, so
--    10^10 <= p_0^2 < n <= 10^10 - contradiction)
-- 2) p_0 < 10^5 -> n is smooth iff P > p_0, we will count nonsmooth numbers

-- Count all products less than maxProd using primes not higher than maxPrime
spr :: Int -> [Int] -> Int -> Int
spr _ [] _ = 1
spr prod (p:ps) maxProd = if p * prod > maxProd
    then 1
    else spr (prod * p) (p:ps) maxProd + spr prod ps maxProd
countProducts :: Int -> Int -> Int
countProducts maxPrime = spr 1 (takeWhile (<=maxPrime) primes)

case2count = countProducts bound (bound^2)
case2nonSmooth = sum $ map (\p -> countProducts p p) primes

euler668 :: IO String
euler668 = return $ show $ case2count - case2nonSmooth