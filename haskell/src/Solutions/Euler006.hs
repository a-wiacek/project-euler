module Solutions.Euler006 where

euler006 :: IO String
euler006 = return $ show $ (^2) (sum [1..100]) - sum (map (^2) [1..100])