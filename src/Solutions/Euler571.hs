module Solutions.Euler571 where
import Utils.List(permutations)
import Utils.Numeric(digitsInBase)

-- Execution time: 2208.0214899s

base = 12

toNum :: [Int] -> Int
toNum = foldl (\x y -> x * base + y) 0

-- We are checking down to base 4, since if number n is pandigital in base b^2
-- it is also pandigital in base b.

euler571 :: IO String
euler571 = return $ show $ sum $ take 10 $ filter (\n -> all (all (>0) . digitsInBase n) [base - 1, base - 2..4])
                  $ map toNum $ filter ((>0) . head) $ permutations [0..base - 1]