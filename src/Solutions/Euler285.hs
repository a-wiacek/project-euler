module Solutions.Euler285 where
import Text.Printf

expected1to10 = 10.20914 :: Double
f x = let u = sqrt (x - 1) in atan u * (x + 1) - atan (1 / u) * (x - 1) - 2 * u
expected n = let k = fromIntegral n in (f ((k + 1 / 2)^2) - f ((k - 1 / 2)^2)) / (2 * k)

euler285 :: IO String
euler285 = return $ printf "%.5f" $ (expected1to10+) $ sum $ map expected [11..100000]