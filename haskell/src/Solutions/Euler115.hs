module Solutions.Euler115 where
import Utils.Array(funArray)
import Data.Array

m = 50 :: Int
bound = 10 * m :: Int

count :: Array Int Int
count = funArray 0 bound f

f :: Int -> Int
f n | n < m = 1
    | otherwise = count ! (n - 1) + 1
                + sum (map (\y -> count ! (n - y - 1)) [m..n - 1])

euler115 :: IO String
euler115 = return $ show $ fst $ head $ dropWhile ((<=10^6) . snd) $ assocs count