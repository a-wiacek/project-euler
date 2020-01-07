module Solutions.Euler214 where
import Utils.NumberTheory(totientArrayUpTo, unPrime, primesUpTo)
import Utils.Array(funArray)
import Data.Array as B
import Data.Array.Unboxed as U

bound = 40000000
totient = totientArrayUpTo bound

chainLength :: Array Int Int
chainLength = funArray 1 bound f

f :: Int -> Int
f 1 = 1
f n = 1 + chainLength B.! (totient U.! n)

euler214 :: IO String
euler214 = return $ show $ sum $ filter ((==25) . (chainLength U.!)) $ map unPrime $ primesUpTo bound