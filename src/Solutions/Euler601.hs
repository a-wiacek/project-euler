module Solutions.Euler601 where

-- streak(n) = k iff n = 1 mod lcm(2, .., s) and n /= 1 mod (s + 1)

-- Number of integers n in range 1 < n < bound such that n = 1 mod lcm(2, .., s)
f :: Integer -> Integer -> Integer
f s bound = (bound - 2) `div` (foldr lcm 1 [2..s])

funP :: Integer -> Integer -> Integer
funP s bound = f s bound - f (s + 1) bound

euler601 :: IO String
euler601 = return $ show $ sum $ map (\i -> funP i (4^i)) [1..31]