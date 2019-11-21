module Solutions.Euler086 where
import Utils.Numeric(isInt)

-- assuming that a >= b >= c, we have:
dist :: Int -> Int -> Int -> Double
dist a b c = sqrt $ fromIntegral $ a^2 + (b + c)^2

computation :: Int -> Int -> Int
computation m total = if total > 1000000
    then m - 1
    else computation (m + 1) (total + length l) where
        l = [() | b <- [1..m], c <- [1..b], isInt $ dist m b c]

euler086 :: IO String
euler086 = return $ show $ computation 1 0