module Solutions.Euler094 where

update :: (Int, Int) -> (Int, Int)
update (x, y) = (2 * x + 3 * y, x + 2 * y)
bound = 10^9 :: Int
case1 :: (Int, Int) -> Int
case1 (x, y) =
    let n = 4 * y * y + 1
        perimeter = 3 * n + 1
    in if perimeter > bound
        then 0
        else perimeter + case1 (update (x, y))

case2 :: (Int, Int) -> Int
case2 (x, y) =
    let n = 2 * (x + 1) `div` 3 - 1
        perimeter = 3 * n - 1
    in if perimeter > bound
        then 0
        else perimeter + case2 (update . update $ (x, y))

euler094 :: IO String
euler094 = return $ show $ case1 (2, 1) + case2 (update . update $ (2, 1))