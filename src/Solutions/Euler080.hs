module Solutions.Euler080 where
import Utils.Numeric(binsearch)
import Data.List
import Data.Char

v = 10^101 :: Integer
findSqRoot :: Integer -> Integer
findSqRoot n = binsearch (\x -> x * x <= n * v * v) v (n * v)

numbers = [2..99] \\ map (^2) [2..9]
compute = sum $ map (sum . take 100 . map digitToInt . show . findSqRoot) numbers

euler080 :: IO String
euler080 = return $ show compute 