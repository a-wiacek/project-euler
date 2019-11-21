module Solutions.Euler304 where
import Utils.Matrix
import Utils.NumberTheory

p = 1234567891011 :: Integer

baseM :: Matrix Integer
baseM = fromLists [[1, 1], [1, 0]]

fib :: Integer -> Integer
fib n = getElem 2 1 (matrixFastPowerMod baseM n p)

euler304 :: IO String
euler304 = return $ show $ sum (map (fib . unPrime) $ take 100000 [nextPrime (10^14) ..]) `mod` p