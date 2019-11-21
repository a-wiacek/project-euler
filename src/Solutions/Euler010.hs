module Solutions.Euler010 where
import Utils.NumberTheory(unPrime, primesUpTo)

euler010 :: IO String
euler010 = return $ show $ sum $ map unPrime $ primesUpTo (2000000 :: Int)