module Solutions.Euler019 where
import Control.Monad.State

dayInMonth :: Int -> Int -> Int
dayInMonth month year
    | month `elem` [0, 2, 4, 6, 7, 9, 11] = 31
    | month `elem` [3, 5, 8, 10] = 30
    | year `mod` 4 == 0 = 29
    | otherwise = 28

type Date = (Int, Int, Int, Int)
addSunday, step :: Date -> Date
addSunday (w, m, y, s) = (w, m, y, s + 1)
step (w, m, y, s) = ((w + dayInMonth m y) `mod` 7, m', y', s) where
    (m', y') = if m == 11 then (0, y + 1) else (m + 1, y)
                
-- Monday = 0, Tuesday = 1, ..., Sunday = 6
-- January = 0, ..., December = 11
loop :: State Date Int
loop = get >>= \(weekday, _, year, sundays) -> if year == 2001
        then return sundays
        else when (weekday == 6) (modify addSunday) >> modify step >> loop

euler019 :: IO String
euler019 = return $ show $ evalState loop (1, 0, 1901, 0)