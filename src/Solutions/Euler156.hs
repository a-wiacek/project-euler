module Solutions.Euler156 where

ifi :: Bool -> Int
ifi True = 1
ifi False = 0

countDigits :: Int -> Int -> Int
countDigits n d = go n 0 where
    go 0 acc = acc
    go r acc = let (m, g) = r `quotRem` 10
               in go (r `div` 10) (acc + ifi (g == d))

f :: Int -> Int -> Int
f n d
    | m == 0 = e
    | otherwise = 10 * f m d - countDigits m d * (9 - g) + m + e
    where (m, g) = n `quotRem` 10
          e = ifi $ g >= d

iter :: Int -> Int
iter d = go (10^11) 0 where
    go n acc | n < 10 = acc
             | n == fnd = go (n - 1) (acc + n)
             | n > fnd = go fnd acc
             | otherwise = go (n - max 1 ((fnd - n) `div` length (show n))) acc
             where fnd = f n d

euler156 :: IO String
euler156 = return $ show $ succ $ sum $ map iter [1..9]