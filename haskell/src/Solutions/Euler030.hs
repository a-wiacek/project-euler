module Solutions.Euler030 where
import Data.Char

fifth :: Int -> Int
fifth = sum . map ((^5) . digitToInt) . show

euler030 :: IO String
euler030 = return $ show $ sum $ filter (\x -> fifth x == x) [2..1000000]