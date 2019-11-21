module Solutions.Euler202 where

-- Number of pairs (x, y) with properties:
-- 1) x + y = (12017639147 + 3) / 2 = 6008819575
-- 2) x == y mod 3 -> x == y == 2 mod 3
-- 3) gcd x y == 1

solutions n = let n' = (n + 3) `div` 2
              in length $ filter ((==1) . gcd n') [2, 5..n']

euler202 :: IO String
euler202 = return $ show $ solutions 12017639147 