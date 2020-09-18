module Solutions.Euler215 where
import Control.Monad
import Data.Array
import Data.Array.ST

compute :: Int -> Int -> Int
compute bound maxH =
    let walls = let walls' n = case compare n bound of
                        LT -> map (n + 2 :) (walls' (n + 2)) ++ map (n + 3 :) (walls' (n + 3))
                        EQ -> [[]]
                        GT -> []
                in map init $ walls' 0
        l = length walls -- for bound = 32 it is 3329
        wallsArray :: Array Int [Int]
        wallsArray = listArray (1, l) walls
        crack :: [Int] -> [Int] -> Bool
        crack [] _ = True
        crack _ [] = True
        crack (h1 : t1) (h2 : t2) = case compare h1 h2 of
            LT -> crack t1 (h2 : t2)
            EQ -> False
            GT -> crack (h1 : t1) t2
        correct :: Array Int [Int]
        correct = listArray (1, l) [filter (\y -> crack (wallsArray ! x) (wallsArray ! y)) [1..l] | x <- [1..l]]
        -- bricks (h, x) = number of valid walls of height h with (crack ! x) at the top
        bricks :: Array (Int, Int) Int
        bricks = runSTArray $ do
            arr <- newArray ((1, 1), (maxH, l)) 1
            forM_ [2..maxH] $ \h -> forM_ [1..l] $ \x ->
                sum <$> forM (correct ! x) (\y -> readArray arr (h - 1, y)) >>= writeArray arr (h, x)
            return arr
    in sum $ map (\x -> bricks ! (maxH, x)) [1..l]

euler215 :: IO String
euler215 = return $ show $ compute 32 10