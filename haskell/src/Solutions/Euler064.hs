module Solutions.Euler064 where

frac q num den = h:t where
    h = (truncate (sqrt $ fromIntegral q) + num) `div` den
    a = num - den * h
    t = if (den == 1 && num /= 0) || q == a * a
            then []
            else frac q (-a) ((q - a * a) `div` den)
cont q = frac q 0 1

euler064 :: IO String
euler064 = return $ show $ length $ filter (even . length) $ map cont [2..9999]