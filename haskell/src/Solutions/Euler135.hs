module Solutions.Euler135 where
import Utils.NumberTheory(divisors)

-- If x = y + d and z = y - d, then we have n = y(4y - d)

solutions n = [() | y <- divisors n, let d4 = n `div` y + y, d4 `mod` 4 == 0 && d4 < 4 * y]

isOk = (==10) . length . solutions

euler135 :: IO String
euler135 = return $ show $ length $ filter isOk [1..999999]