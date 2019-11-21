module Solutions.Euler203 where
import Utils.List(uniques)
import Utils.NumberTheory(unPrime, primes)
import Utils.Numeric(binom)

isSquareFree :: Integer -> Bool
isSquareFree n =
    let check n (p:ps)
            | p * p > n = True
            | n `mod` p == 0 =
                let n' = n `div` p
                in (n' `mod` p /= 0) && check n' ps
            | otherwise = check n ps
    in check n $ map unPrime primes

compute :: Integer
compute = sum $ uniques $ filter isSquareFree [binom x y | x <- [0..50], y <- [0..x]]

euler203 :: IO String
euler203 = return $ show compute