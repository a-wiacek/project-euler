module Solutions.Euler157 where
import Utils.List(uniques)
import Utils.NumberTheory(divisors)

solutions :: Int -> Int
solutions n = let p10 = 10^n in length $ uniques [(pa `div` p, pb `div` p, p, n) |
    x <- [0..2 * n],
    y <- [0..2 * n],
    let d = 2^x * 5^y,
    d <= p10,
    let pa = d + p10,
    let pb = p10 * p10 `div` d + p10,
    p <- divisors $ gcd pa pb]

euler157 :: IO String
euler157 = return $ show $ sum $ map solutions [1..9]