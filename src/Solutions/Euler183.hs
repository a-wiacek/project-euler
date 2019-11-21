module Solutions.Euler183 where
-- Maximum is achieved at k closest to n/e.

uberdiv n k = if n `mod` k == 0 then uberdiv (n `div` k) k else n
smallest :: Integer -> Integer
smallest n =
    let k = round $ fromIntegral n / exp 1
    in if (k `div` gcd n k) `uberdiv` 2 `uberdiv` 5 == 1 then -n else n

euler183 :: IO String
euler183 = return $ show $ sum $ map smallest [5..10000]