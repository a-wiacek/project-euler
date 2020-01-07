module Solutions.Euler037 where
import Utils.NumberTheory(primesArrayUpTo)
import Data.Array.Unboxed((!))

good n pivot = all (\f -> primes ! read (f pivot n)) [take, drop]

bound = 799999
primes = primesArrayUpTo bound
superPrime n =
    let s = let n' = show n in filter (good n') [1..length n' - 1]
    in primes ! n && length (show n) - 1 == length s

euler037 :: IO String
euler037 = return $ show $ sum $ filter superPrime [10..bound]