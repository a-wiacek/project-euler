module Solutions.Euler007 where
import Utils.NumberTheory(unPrime, primes)

euler007 :: IO String
euler007 = return $ show $ unPrime $ primes !! 10000