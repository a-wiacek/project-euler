module Solutions.Euler274 where
import Data.Maybe(mapMaybe)
import Utils.NumberTheory(unPrime, invertMod, primesUpTo)

funF :: Int -> Int
funF = sum . mapMaybe (invertMod 10 . unPrime) . primesUpTo . pred

euler274 :: IO String
euler274 = return $ show $ funF $ 10^7