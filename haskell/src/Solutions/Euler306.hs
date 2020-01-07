module Solutions.Euler306 where

-- http://oeis.org/A215721
-- Found using simulation
-- The most important line is:
-- For n > 14, a(n) = a(n - 5) + 34

initSequence, period, totalSequence :: [Int]
initSequence = [1, 5, 9, 15, 21, 25, 29, 35]
period = [39, 43, 55, 59, 63]
totalSequence = initSequence ++ concatMap (\s -> map (+s) period) [0, 34..]

funF :: Int -> Int
funF n = n - length (takeWhile (<=n) totalSequence)

euler306 :: IO String
euler306 = return $ show $ funF $ 10^6