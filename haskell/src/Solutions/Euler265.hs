module Solutions.Euler265 where
import Data.Char(digitToInt, intToDigit)
import Data.List(foldl')
import Control.Monad
import Numeric(showIntAtBase)

dec2bin :: Int -> String
dec2bin n = showIntAtBase 2 intToDigit n ""

bin2dec :: String -> Int
bin2dec = foldl' (\acc x -> acc * 2 + digitToInt x) 0

deBrujinSequences :: Int -> [String]
deBrujinSequences n =
    let npow = 2^n
        shift :: String -> String
        shift s = replicate (n - 1) '0' ++ take (npow - n + 1) s
        neighbours :: Int -> (Int, Int)
        neighbours n = (k, k + 1) where k = 2 * n `mod` npow
        translate :: [Int] -> String
        translate = shift . map (intToDigit . (`mod` 2))
        search :: [Int] -> [[Int]]
        search l@(h:t)
            | h `elem` t = []
            | length l == npow = [reverse l]
            | otherwise = search (x:l) ++ search (y:l)
            where (x, y) = neighbours h
    in map translate $ search [0]

euler265 :: IO String
euler265 = return $ show $ sum $ map bin2dec $ deBrujinSequences 3