module Solutions.Euler221 where
import Utils.List(uniques)
import Utils.NumberTheory(divisors)

-- http://oeis.org/A147811
alexandrian :: Int -> [Integer]
alexandrian n = uniques $ do
    p' <- [1..n]
    d <- map toInteger $ divisors $ p'^2 + 1
    let p = toInteger p'
    return $ p * (p + d) * (p + (p^2 + 1) `div` d)

euler221 :: IO String
euler221 = return $ show $ alexandrian 100000 !! 149999