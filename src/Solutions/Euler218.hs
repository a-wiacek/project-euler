module Solutions.Euler218 where

f (x, y) = (x^2 - y^2 - 2 * x * y) * (x^2 - y^2 + 2 * x * y) * (x - y) * (x + y) * 2 * x * y
pairs = [(x, y) | x <- [0..6], y <- [0..6]]

euler218 :: IO String
euler218 = return $ show $ filter (\x -> x `mod` 7 /= 0) $ map f pairs