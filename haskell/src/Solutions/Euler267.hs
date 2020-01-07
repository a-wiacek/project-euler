module Solutions.Euler267 where
import Utils.Numeric(binom)
import Text.Printf

euler267 :: IO String
euler267 = return $ printf "%.12f" (fromIntegral (sum $ map (binom 1000) [432..1000]) / (2^1000) :: Double)