module Solutions.Euler204 where
import Utils.NumberTheory(unPrime, primesUpTo)

hamming' :: Int -> [Int] -> Int -> Int
hamming' n [] bound = if n <= bound then 1 else 0
hamming' n (p:ps) bound
    | n > bound = 0
    | n * p > bound = 1
    | otherwise = hamming' n ps bound + hamming' (n * p) (p:ps) bound

hamming :: Int -> Int -> Int
hamming bound hType = hamming' 1 (map unPrime $ primesUpTo hType) bound

euler204 :: IO String
euler204 = return $ show $ hamming (10^9) 100