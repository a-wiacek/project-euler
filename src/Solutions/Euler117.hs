module Solutions.Euler117 where
import Utils.Array(funArray)
import Data.Array

bound = 50 :: Int
arr :: Array Int Int
arr = funArray 0 bound f

f :: Int -> Int
f n | n < 4 = [1, 1, 2, 4] !! n
    | otherwise = sum $ map (\x -> arr ! (n - x)) [1..4]

euler117 :: IO String
euler117 = return $ show $ arr ! bound