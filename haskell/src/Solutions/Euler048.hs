module Solutions.Euler048 where

euler048 :: IO String
euler048 = return $ show $ sum (map (\x -> x^x) [1..1000]) `mod` 10000000000