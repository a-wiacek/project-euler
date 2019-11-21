module Solutions.Euler114 where
import Utils.Array(funArray)
import Data.Array

bound = 50 :: Int

count :: Array Int Int
count = funArray 0 bound f

f :: Int -> Int
f x | x < 3 = 1
    | otherwise = count ! (x - 1) + 1 -- grey block or red block of maximum length
                + sum (map (\y -> count ! (x - y - 1)) [3..x - 1]) -- red block of length y and grey block after

euler114 :: IO String
euler114 = return $ show $ count ! 50