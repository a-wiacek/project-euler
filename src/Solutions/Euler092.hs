module Solutions.Euler092 where
import Data.Array.Unboxed
import Data.Char

bound = 9999999 :: Int
lowBound = 999 :: Int
sumSquares :: Int -> Int
sumSquares = sum . map ((^2) . digitToInt) . show

-- True is for 89, False is for 1
computeSmall n = case sumSquares n of
    1 -> False
    89 -> True
    s -> computeSmall s

baseValues :: UArray Int Bool
baseValues = array (1, lowBound) [(x, computeSmall x) | x <- [1..lowBound]]

computeLarge n = let s = sumSquares n in
    if s > lowBound
        then computeLarge s
        else baseValues ! s

euler092 :: IO String
euler092 = return $ show $ length $ filter computeLarge [1..bound]