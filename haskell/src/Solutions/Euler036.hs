module Solutions.Euler036 where
import Utils.Numeric(isPalindrome)
import Utils.Operators((<&&>))

toBin :: Integer -> Integer
toBin = read . toBin' where
    toBin' 0 = "0"
    toBin' 1 = "1"
    toBin' n = toBin' (n `div` 2) ++ if n `mod` 2 == 0 then "0" else "1"

euler036 :: IO String
euler036 = return $ show $ sum $ filter (isPalindrome <&&> isPalindrome . toBin) [1..999999]