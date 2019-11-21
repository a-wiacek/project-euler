module Solutions.Euler119 where
import Utils.List(uniques)
import Data.Char(digitToInt)

sumOfDigits = toInteger . sum . map digitToInt . show

l :: [Integer]
l = uniques [s | a <- [1..100], b <- [1..50], let s = a^b,
                 s >= 10, sumOfDigits s == a]

euler119 :: IO String
euler119 = return $ show $ l !! 29