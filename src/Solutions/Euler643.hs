module Solutions.Euler643 where
import Math.NumberTheory.MoebiusInversion

funF :: Int -> Integer
funF = sum . map (\x -> totientSum x - 1) . takeWhile (>1) . iterate (`div` 2) . (`div` 2)

euler643 :: IO String
euler643 = return $ show $ funF (10^11) `mod` 1000000007