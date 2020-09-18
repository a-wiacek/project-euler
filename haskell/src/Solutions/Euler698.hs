module Solutions.Euler698 where
import Data.List
import Utils.Numeric

-- We can estimate shape of the number:

possibleDigitCounts = [0, 1, 2, 3, 11, 12, 13, 21, 22, 23, 31, 32, 33]
possibleDigitsCounts = groupBy (\l1 l2 -> sum l1 == sum l2) $ sortOn sum
    [[x, y, z] | x <- possibleDigitCounts,
                 y <- possibleDigitCounts,
                 x <= y,
                 z <- possibleDigitCounts,
                 y <= z,
                 z /= 0] -- omit [0, 0, 0]
countNums l = multinom l * toInteger (length $ nub $ permutations l)
countNumsOfLen = sum . map countNums
limit = 111111111111222333
stepFun (_, n, _) l = let s = n + countNumsOfLen l in (sum (head l), s, s > limit)
enumerate = scanl stepFun (0, 0, False) possibleDigitsCounts

{- Fragment of the output of demo: (mapM_ print enumerate)

(36,29976051355052100,False)
(37,85645659280376850,False)
(38,170250467385927030,True)
(39,254855275491477210,True)

This means that F(111111111111222333) has 38 digits and it is
111111111111222333 - 85645659280376850 = 25465451830845483th such number out of
170250467385927030 - 85645659280376850 = 84604808105550180.

Let us inspect patterns for 123-numbers with 38 digits.
-}

patternsOf38Query = find ((==38) . sum . head) possibleDigitsCounts

-- This gives us:

patternsOf38 = [[2,3,33],[2,13,23],[3,3,32],[3,12,23],[3,13,22],[12,13,13]]

euler698 :: IO String
euler698 = return undefined