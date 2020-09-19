{-# LANGUAGE Strict #-}
module Solutions.Euler287 where

funF :: Int -> Int
funF n = go bounds bounds where
    d = 2^(n - 1)
    bounds = [[0, d - 1], [d, d + d - 1]]
    black x y = (x - d)^2 + (y - d)^2 <= d * d
    split [x, y] = [[x, mid], [mid + 1, y]] where mid = (x + y) `div` 2
    go x y = succ $ sum $ count <$> x <*> y
    count x y = case black <$> x <*> y of
        [True, True, True, True] -> 2
        [False, False, False, False] -> 2
        _ -> go (split x) (split y)

euler287 :: IO String
euler287 = return $ show $ funF 24