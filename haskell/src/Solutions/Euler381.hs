module Solutions.Euler381 where
import Utils.NumberTheory(unPrime, primesUpTo, invertMod)
import Data.Maybe(fromJust)

inv p a = fromJust $ invertMod a p
-- Wilson theorem
s :: Integer -> Integer
s p = let [a1, a2, a3, a4] = map (inv p . (p-)) [1, 2, 3, 4]
      in (-1 - a1 - a1 * a2 - a1 * a2 * a3 - a1 * a2 * a3 * a4) `mod` p

f = sum . map (s . unPrime) . drop 2 . primesUpTo

euler381 :: IO String
euler381 = return $ show $ f $ 10^8