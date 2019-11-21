module Solutions.Euler073 where

-- -2, since this counts 1/2 and 1/3
count = length [() |
    den <- [2..12000],
    num <- [ceiling $ fromIntegral den / 3..floor $ fromIntegral den / 2],
    gcd den num == 1] - 2

euler073 :: IO String
euler073 = return $ show count