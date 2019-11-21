module Solutions.Euler106 where
import Utils.Numeric(binom)

f n s = binom n s * binom (n - s) s `div` 2 - binom (2 * s) s * binom n (2 * s) `div` (s + 1)

euler106 :: IO String
euler106 = return $ show $ sum $ map (f 12) [2..6]