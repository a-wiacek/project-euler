module Solutions.Euler078 where
import Utils.Array(funArray)
import Data.Array

-- Pentagonal number theorem
pentagonal :: [(Int, Int)]
pentagonal = pen 1 where
    pen k = [((-1)^(k - 1), k * (3 * k - 1) `div` 2),
             ((-1)^(k - 1), k * (3 * k + 1) `div` 2)] ++ pen (k + 1)

bound = 75000 :: Int
partitions :: Array Int Int
partitions = funArray 0 bound f

f :: Int -> Int
f 0 = 1
f n = sum (map (\(mul, y) -> mul * partitions ! (n - y))
    $ takeWhile ((<=n) . snd) pentagonal) `mod` 1000000

euler078 :: IO String
euler078 = return $ show $ fst $ head $ filter ((==0) . snd) $ assocs partitions