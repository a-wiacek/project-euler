module Solutions.Euler066 where
import Solutions.Euler064(cont)
import Solutions.Euler065(approxs)
import Data.List
import Data.Ratio

isBad n r = numerator r ^2 - n * denominator r ^2 /= 1 
minimumX n = (n, numerator $ head $ dropWhile (isBad n) $ approxs infrac)
    where h:t = cont n
          infrac = h : cycle t

numbers = [2..1000] \\ map (^2) [2..31]
max' (a, b) (c, d) = if b > d then (a, b) else (c, d)

euler066 :: IO String
euler066 = return $ show $ fst $ foldr1 max' $ map minimumX numbers