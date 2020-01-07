module Solutions.Euler173 where

laminae l t
    | 4 * l - 4 > 1000000 = 0
    | 4 * t * (l - t) > 1000000 || 2 * t >= l = laminae (l + 1) 1
    | otherwise = 1 + laminae l (t + 1)

euler173 :: IO String
euler173 = return $ show $ laminae 3 1