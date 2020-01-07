module Solutions.Euler057 where

update (num, den, ctr) = (num + 2 * den, num + den, if p then ctr + 1 else ctr) where
    p = length (show num) /= length (show den)
get (_, _, c) = c

euler057 :: IO String
euler057 = return $ show $ get $ iterate update (1, 1, 0) !! 1000