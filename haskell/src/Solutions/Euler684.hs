module Solutions.Euler684 where
import Utils.Numeric(fibonacci, fastPowerMod)

{-
1 2 3 4 5 6 7 8 9  | 10 11 12 13 14 15 16 17 18  | 19  20  21  22  ...
1 2 3 4 5 6 7 8 9  | 19 29 39 49 59 69 79 89 99  | 199 299 399 499 ...
2 3 4 5 6 7 8 9 10 | 20 30 40 50 60 70 80 90 100 | 200 300 400 500 ...
If n = 9k + l (l \in [1..9]), then s(n) = (l + 1) * 10^k - 1
Let N = 9K + L (L \in [1..9]). Then:
S(N) = \sum_{i = 1}^{9K + L} s(i)
     = \sum_{i = 0}^{K - 1} \sum_{j = 1}^9 ((j + 1) * 10^i - 1) + \sum_{j = 1}^L ((j + 1) * 10^K - 1)
     = ... = 10^K * (6 + L * (L + 3) / 2) - 6 - 9 * K - L
Example: N = 20 -> K = 2, L = 2
             100 * (6 + 5) - 6 - 18 - 2
-}

p = 10^9 + 7 :: Integer

funS :: Integer -> Integer
funS n = fastPowerMod 10 k p * (6 + l * (l + 3) `div` 2) - 6 - 9 * k - l
    where k = (n - 1) `div` 9
          l = n - 9 * k

euler684 :: IO String
euler684 = return $ show $ (`mod` p) $ sum $ map funS $ take 89 $ fibonacci 1 2