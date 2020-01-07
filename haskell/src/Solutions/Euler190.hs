module Solutions.Euler190 where

maxP m' =
    let m = fromIntegral m'
    in floor $ product (map (\x -> x**x) [2..m]) * (2 / (m + 1))**(m * (m + 1) / 2)

euler190 :: IO String
euler190 = return $ show $ sum $ map maxP [2..15]