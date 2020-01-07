module Solutions.Euler357 where
import Utils.NumberTheory(primesArrayUpTo)
import Data.Array.Unboxed

bound = 10^8
primes = primesArrayUpTo $ bound + 1
-- Suppose that n = p^2 k, where p is prime.
-- Then p is divisor of n and p + n/p = p + p * k = p (k + 1), so
-- n does not have desired property.
-- Divisor of n is 1, so 1 + n / 1 = n + 1 is prime.
-- n must be even, so n = 4k + 2 for some k.

isPrime = (!) primes :: Int -> Bool
sqrt' = floor . sqrt . fromIntegral :: Int -> Int

isOk n = all isPrime $ map (\d -> d + n `div` d) $ filter ((==0) . mod n) [1..sqrt' n]

euler357 :: IO String
euler357 = return $ show $ succ $ sum $ filter isOk [2, 6..bound]