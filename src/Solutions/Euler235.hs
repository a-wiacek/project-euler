module Solutions.Euler235 where
import Utils.Numeric(bisection)
import Text.Printf

f :: Double -> Double
f r = let s = r^5000 in 300 * (s - 1) * (r - 1) - s * (r * 5000 - 5001) - 1 + (r - 1)^2 * 200000000000

euler235 :: IO String
euler235 = return $ printf "%.12f" $ bisection f 40 1.001 1.003