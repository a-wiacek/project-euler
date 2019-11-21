module Solutions.Euler007 where
import Utils.NumberTheory(primes)

euler007 :: IO String
euler007 = return $ show $ primes !! 10000