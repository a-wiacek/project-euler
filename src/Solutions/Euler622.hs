module Solutions.Euler622 where
import Utils.NumberTheory(divisors)

-- http://oeis.org/A002326

runShuffleEq :: Int -> Int -> Bool
runShuffleEq bound n = go (bound - 1) 2
    where go left l | left <= 0 = l == 1
                    | l == 1 = False
                    | otherwise = go (left - 1) ((2 * l) `mod` n)

sumS :: Int -> Int
sumS n = sum $ map succ $ filter (runShuffleEq n) $ tail $ divisors (2^n - 1)

euler622 :: IO String
euler622 = return $ show $ sumS 60