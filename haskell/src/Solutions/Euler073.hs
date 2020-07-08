module Solutions.Euler073 where

-- ceiling (a / b) = floor ((a + b - 1) / b)
-- -2, since this counts 1/2 and 1/3
count = length [() |
    den <- [2 .. 12000],
    num <- [(den + 2) `div` 3 .. den `div` 2],
    gcd den num == 1] - 2

euler073 :: IO String
euler073 = return $ show count