module Solutions.Euler349 where

a10024 = 724 :: Integer

euler349 :: IO String
euler349 = return $ show $ 12 * (10^18 - 10024) `div` 104 + a10024