module Solutions.Euler291 where
import Math.NumberTheory.Primes.Testing(isPrime)

-- https://oeis.org/A027862
euler291 :: IO String
euler291 = return $ show $ length $ filter isPrime $ takeWhile (<5 * (10^15)) $ map (\n -> n^2 + (n + 1)^2) [1..]