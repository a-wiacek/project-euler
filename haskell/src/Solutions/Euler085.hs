module Solutions.Euler085 where

opt = 2000000
best (a1, r1) (a2, r2) = if abs (r1 - opt) < abs (r2 - opt) then (a1, r1) else (a2, r2)

euler085 :: IO String
euler085 = return $ show $ fst
         $ foldr1 best [(i * j, i * (i + 1) * j * (j + 1) `div` 4) | i <- [1..2000], j <- [i..2000]]