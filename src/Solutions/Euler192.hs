module Solutions.Euler192 where
import Data.Ratio
import Utils.List(ascendingMinus)

bestApprox bound n = go (0, 1) (1,0) where
    go low@(a, b) high@(c, d)
      | q > bound = if distance low < distance high then low else high
      | p * p > q * q * n = go low mid
      | otherwise = go mid high
         where mid@(p, q) = (a + c, b + d)
               distance (p, q) = abs ((p % q)^2 - n % 1)

sumA :: Integer -> Integer -> Integer
sumA n bound = sum $ map (snd . bestApprox bound) $ ascendingMinus [2..n] $ takeWhile (<=n) $ map (^2) [2..]

euler192 :: IO String
euler192 = return $ show $ sumA 100000 (10^12)