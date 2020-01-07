module Solutions.Euler031 where
import Utils.Operators((<:>))
import Data.Array

coins = listArray (0, 7) [1, 2, 5, 10, 20, 50, 100, 200] :: Array Int Int

arr :: Array (Int, Int) Int
arr = array ((-200, 0), (200, 7)) [(id <:> f) (x, y) | x <- [-200..200], y <- [0..7]]

f (x, y) | x < 0 = 0
         | x == 0 || y == 0 = 1
         | otherwise = sum $ map (\ci -> arr ! (x - coins ! ci, ci)) [0..y]

euler031 :: IO String
euler031 = return $ show $ arr ! (200, 7)