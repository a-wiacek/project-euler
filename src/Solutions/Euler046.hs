module Solutions.Euler046 where
import Utils.NumberTheory(primesArrayUpTo)
import Data.Array.Unboxed((!))

primes = primesArrayUpTo 7000

possible n i =
    let n' = n - 2 * i * i
    in n' > 0 && ((primes ! n') || possible n (i + 1))

counterexample n
    | primes ! n = counterexample $ n + 2
    | possible n 1 = counterexample $ n + 2
    | otherwise = n

euler046 :: IO String
euler046 = return $ show $ counterexample 35