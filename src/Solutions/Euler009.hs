module Solutions.Euler009 where

triples :: [[Int]]
triples = [[x, y, z] | x <- [1..444], y <- [x..444], let z = 1000 - x - y, x * x + y * y == z * z] 

euler009 :: IO String
euler009 = return $ show $ product $ head triples