module Solutions.Euler129 where
import Utils.Operators((<:>))
-- A(n) < n

a k | gcd k 10 == 1 = let a' acc n | acc `mod` k == 0 = n
                                   | otherwise = a' ((10 * acc + 1) `mod` k) (n + 1)
                      in a' 1 1
    | otherwise = -1

euler129 :: IO String
euler129 = return $ show $ snd $ head $ dropWhile ((<=10^6) . fst) $ map (a <:> id) [10^6 + 1..]