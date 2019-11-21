module Solutions.Euler120 where

-- See problem 123 for explanation.
remainder a n = if even n then 2 else 2 * a * n `mod` (a * a)
maxRemainder n = foldr (max . remainder n) 0 [0..2 * n]

euler120 :: IO String
euler120 = return $ show $ sum $ map maxRemainder [3..1000]