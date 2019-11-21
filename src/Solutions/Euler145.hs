module Solutions.Euler145 where
import Utils.Operators((<&&>))
import Utils.Numeric(reverseInt)
-- No reason to not brute force.

isOddDigited :: Int -> Bool
isOddDigited 0 = True
isOddDigited n = odd (n `mod` 10) && isOddDigited (n `div` 10)

isReversible :: Int -> Bool
isReversible n = isOddDigited $ n + reverseInt n

f n = length $ filter ((\x -> x `mod` 10 /= 0) <&&> isReversible) [1..n - 1]

euler145 :: IO String
euler145 = return $ show $ f $ 10^9