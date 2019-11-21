module Solutions.Euler686 where
import Data.Maybe

-- 123 * 10^k <= 2^n < 124 * 10^k
-- log_2(123) + k * log_2(10) <= n < log_2(124) + k * log_2(10)

l10 = log 10 / log 2
l123 = log 123 / log 2
l124 = log 124 / log 2

findPower :: Double -> Maybe Int
findPower k =
    let s = k * l10
        low = ceiling $ l123 + s
        high = floor $ l124 + s
    in if low == high
        then Just high
        else Nothing

properPowers :: [Int]
properPowers = mapMaybe findPower [2.0..]

euler686 :: IO String
euler686 = return $ show $ properPowers !! 678909