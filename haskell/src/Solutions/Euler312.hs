module Solutions.Euler312 where
import Utils.NumberTheory(invertMod)
import Utils.Numeric(fastPowerMod)
import Math.NumberTheory.Moduli.Chinese(chineseRemainder)
import Data.Maybe(fromJust)

getB :: Integer -> Integer -> Integer
getB n m = (fromJust (invertMod 18 m) * (n - 27)) `mod` m
c3 = (8 * fastPowerMod 12 ac2 (13^8)) `mod` 13^8
ac2 = fromJust $ chineseRemainder [(b2, 13^7), (3, 12)]
b2 = getB (fastPowerMod 3 d2 (13^7)) (13^7)
d2 = fromJust $ chineseRemainder [(c2, 13^6), (0, 12)]
c2 = (8 * fastPowerMod 12 ac1 (13^6)) `mod` 13^6
ac1 = fromJust $ chineseRemainder [(b1, 13^5), (3, 12)]
b1 = getB (fastPowerMod 3 d1 (13^5)) (13^5)
d1 = fromJust $ chineseRemainder [(c1, 13^4), (0, 12)]
c1 = (8 * fastPowerMod 12 ac0 (13^4)) `mod` 13^4
ac0 = fromJust $ chineseRemainder [(b0, 13^3), (3, 12)]
b0 = getB (fastPowerMod 3 d0 (13^3)) (13^3)
d0 = 10000 `mod` (13^2 * 12)

euler312 :: IO String
euler312 = return $ show c3