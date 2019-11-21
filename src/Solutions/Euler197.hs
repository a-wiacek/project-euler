module Solutions.Euler197 where

f x = 10**(-9) * fromIntegral (floor $ 2 ** (30.403243784 - x * x))
u = map (floor . (*(10^9))) $ iterate f (-1)
-- Playing with u and uniques function (from EulerUtils), one can see, that
-- sequence eventually stabilizes to be periodic with period of length 2.

[a, b] = take 2 $ drop 600 u

euler197 :: IO String
euler197 = return $ show $ fromIntegral (a + b) / (10^9)