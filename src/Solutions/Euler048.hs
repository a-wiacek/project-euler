module Solutions.Euler048 where

euler048 :: IO String
euler048 = return $ reverse $ take 10 $ reverse $ show $ sum $ map (\x -> x^x) [1..1000]