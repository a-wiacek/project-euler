module Solutions.Euler005 where

euler005 :: IO String
euler005 = return $ show $ foldr1 lcm [1..20]