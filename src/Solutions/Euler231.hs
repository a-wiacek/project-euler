module Solutions.Euler231 where
import Utils.NumberTheory(unPrime, primes)

factorsSum :: Int -> Int
factorsSum n =
    let fsum :: Int -> Int -> [Int] -> Int
        fsum n ans (p:ps)
            | n == 0 = 0
            | abs n == 1 = ans
            | p * p > n = ans + n
            | n `mod` p == 0 = fsum (n `div` p) (ans + p) (p:ps)
            | otherwise = fsum n ans ps
    in fsum n 0 $ map unPrime primes

factorsBin n m =
    let m' = if m + m > n then n - m else m
    in sum (map factorsSum [n - m' + 1..n]) - sum (map factorsSum [1..m'])

euler231 :: IO String
euler231 = return $ show $ factorsBin 20000000 15000000