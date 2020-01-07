module Solutions.Euler123 where
import Utils.List(everyOther)
import Utils.NumberTheory(unPrime, primes)
import Data.List
import Data.Maybe

isOK (n, p) = 2 * n * unPrime p > 10^10

euler123 :: IO String
euler123 = return $ show $ fst $ fromJust $ find isOK $ everyOther True $ zip [1..] primes