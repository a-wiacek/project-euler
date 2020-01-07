module Solutions.Euler064 where
import Data.List

frac q num den = h:t where
    h = (truncate (sqrt $ fromIntegral q) + num) `div` den
    a = num - den * h
    t = if den == 1 && num /= 0
            then []
            else frac q (-a) ((q - a * a) `div` den)
cont q = frac q 0 1

numbers = [2..9999] \\ map (^2) [2..99]

euler064 :: IO String
euler064 = return $ show $ length $ filter (even . length) $ map cont numbers