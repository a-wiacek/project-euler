module Solutions.Euler179 where
import Utils.NumberTheory(numberOfDivisors)

list = map numberOfDivisors ([2..10^7] :: [Int])
process = go 0 where
    go acc [m, n] = acc + if m == n then 1 else 0
    go acc (m:n:t) = go (acc + if m == n then 1 else 0) (n:t)

euler179 :: IO String
euler179 = return $ show $ process list