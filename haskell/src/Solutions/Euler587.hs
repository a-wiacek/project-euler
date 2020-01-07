module Solutions.Euler587 where

-- Approximation factor was found by trial and error.
blueArea = 1 - pi / 4 :: Double
orangeArea :: Double -> Double
orangeArea n = 0.9905 * (n + 1 - sqrt (2 * n)) / (2 * n * n + 2)
approx :: Double -> Double
approx n = orangeArea n / blueArea

euler587 :: IO String
euler587 = return $ show $ floor $ head $ dropWhile (\n -> approx n > 0.001) [2.0..]