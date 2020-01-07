module Solutions.Euler125 where
import Utils.List(uniques)
import Utils.Numeric(isPalindrome)
-- sum of squares from 1 to n
sumSquares n = n * (n + 1) * (2 * n + 1) `div` 6

compute :: Int
compute = sum $ uniques [k |
    a <- [2..10000],
    b <- [0..a - 2],
    let k = sumSquares a - sumSquares b,
    isPalindrome k,
    k < 10^8]

euler125 :: IO String
euler125 = return $ show compute 