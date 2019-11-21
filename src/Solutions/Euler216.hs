module Solutions.Euler216 where
import Math.NumberTheory.Primes.Testing(isPrime)

euler216 :: IO String
euler216 = return $ show $ length $ filter isPrime $ map (\n -> 2 * n^2 - 1) [2..50000000]