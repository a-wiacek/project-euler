module Solutions.Euler315 where
import Utils.NumberTheory(unPrime, nextPrime, precPrime)
import Utils.Numeric(digitsSum)
import Utils.List(ascendingXor, foldDescend)

data Segment = U | LU | RU | M | LD | RD | D deriving (Eq, Ord)
segments :: Int -> [Segment]
segments 0 = [U, LU, RU, LD, RD, D]
segments 1 = [RU, RD]
segments 2 = [U, RU, M, LD, D]
segments 3 = [U, RU, M, RD, D]
segments 4 = [LU, RU, M, RD]
segments 5 = [U, LU, M, RD, D]
segments 6 = [U, LU, M, LD, RD, D]
segments 7 = [U, LU, RU, RD]
segments 8 = [U, LU, RU, M, LD, RD, D]
segments 9 = [U, LU, RU, M, RD, D]

cost :: Int -> Int
cost = length . segments

totalCost :: Int -> Int
totalCost n = if n == 0 then 0 else cost (n `mod` 10) + totalCost (n `div` 10)

costDiff :: Int -> Int -> Int
costDiff x y = length $ ascendingXor (segments x) (segments y)

totalCostDiff :: Int -> Int -> Int
totalCostDiff x y
    | x == 0 && y == 0 = 0
    | x > 0 && y == 0 = totalCost x
    | y > 0 && x == 0 = totalCost y
    | otherwise = costDiff (x `mod` 10) (y `mod` 10) + totalCostDiff (x `div` 10) (y `div` 10)

energyCalc :: (Int -> Int -> Int) -> [Int] -> Int
energyCalc transitionCost numbers = totalCost (head numbers)
                                  + totalCost (last numbers)
                                  + sum (foldDescend transitionCost numbers)

digitalRootSteps :: Int -> [Int]
digitalRootSteps n = if n < 10 then [n] else n : digitalRootSteps (digitsSum n)

nums = map (digitalRootSteps . unPrime) [nextPrime 10000000 .. precPrime 20000000]
samCost = sum $ map (energyCalc (\x y -> totalCost x + totalCost y)) nums
maxCost = sum $ map (energyCalc totalCostDiff) nums

euler315 :: IO String
euler315 = return $ show $ samCost - maxCost