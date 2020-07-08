module Solutions.Euler019 where

dayInMonth :: Int -> Int -> Int
dayInMonth month year
    | month `elem` [0, 2, 4, 6, 7, 9, 11] = 31
    | month `elem` [3, 5, 8, 10] = 30
    | year `mod` 4 == 0 = 29
    | otherwise = 28

data Date = Date { weekday :: !Int, month :: !Int, year :: !Int }
                
-- Monday = 0, Tuesday = 1, ..., Sunday = 6
-- January = 0, ..., December = 11

step :: Date -> Date
step (Date w m y) = Date ((w + dayInMonth m y) `mod` 7) m' y' where
    (m', y') = if m == 11 then (0, y + 1) else (m + 1, y)

query :: Date -> Int
query = length . filter ((== 6) . weekday) . takeWhile ((<= 2000) . year) . iterate step

euler019 :: IO String
euler019 = return $ show $ query $ Date 1 0 1901