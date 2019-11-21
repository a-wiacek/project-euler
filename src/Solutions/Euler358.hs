module Solutions.Euler358 where
import Data.List(find)
import qualified Data.Map.Strict as M
import Utils.NumberTheory
import Utils.Numeric(fastPowerMod, digitsSum)
import Utils.Operators

f = fastPowerMod 10
fromJust (Just x) = x

primRoot :: Prime Int -> Bool
primRoot n' = all (\p -> f (phi `div` unPrime p) n /= 1) ps 
    where n = unPrime n'
          phi = n - 1
          ps = M.keys $ runFactorization $ factorize phi

properEnd :: Prime Int -> Bool
properEnd n' = ((f (n - 1) d - 1) * fromJust (invertMod n d)) `mod` d == 56789
    where n = unPrime n'
          d = 100000

uniquePrime :: Int
uniquePrime = unPrime $ fromJust $ find
                 (primRoot <&&> properEnd)
                 [nextPrime 724637680 .. precPrime 729927080]
                 -- these bounds are for 00000000137 part, found by plugging 10^11/137 and 10^11/138 to WolframAlpha

cyclicDigitsSum :: Int -> Int
cyclicDigitsSum n = go 0 9 where
    go acc rem
        | r == 0 = acc + q
        | otherwise = go (acc + q) (10 * r + 9) 
        where (q, r) = rem `quotRem` n

euler358 :: IO String
euler358 = return $ show $ cyclicDigitsSum uniquePrime