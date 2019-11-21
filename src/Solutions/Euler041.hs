module Solutions.Euler041 where
import Utils.NumberTheory(primesArrayUpTo)
import Data.Array.Unboxed((!))
import Data.List

-- 8-digit and 9-digit pandigital numbers are not prime, since sum of their
-- digits is 36 or 45 and that means that they are divisible by 3.

primes = primesArrayUpTo 7654321

euler041 :: IO String
euler041 = return $ show $ maximum $ filter (primes !) $ map read $ permutations "1234567"