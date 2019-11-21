module Solutions.Euler011 where
import Utils.Input(getInput)
import Data.Array

column, row, diagonal1, diagonal2 :: Int -> Int -> Int -> [(Int, Int)]
column x y length = [(x, y + t) | t <- [0..length - 1]]
row x y length = [(x + t, y) | t <- [0..length - 1]]
diagonal1 x y length = [(x + t, y - t) | t <- [0..length - 1]]
diagonal2 x y length = [(x + t, y + t) | t <- [0..length - 1]]

arrayLines :: Int -> Int -> Int -> [[(Int, Int)]]
arrayLines xsize ysize length = if length == 1
    then [[(x, y)] | x <- [0..xsize - 1], y <- [0..ysize - 1]] -- avoid duplicates
    else [column x y length | x <- [0..xsize - 1], y <- [0..ysize - length]] ++
         [row x y length | x <- [0..xsize - length], y <- [0..ysize - 1] ] ++
         [diagonal1 x y length | x <- [0..xsize - length], y <- [length - 1..ysize - 1]] ++
         [diagonal2 x y length | x <- [0..xsize - length], y <- [0..ysize - length]]

productOfLine :: Array (Int, Int) Int -> [(Int, Int)] -> Int
productOfLine array = product . map (array !)

compute :: String -> Int
compute s =
    let s' = map (map read . words) $ lines s :: [[Int]]
        arrRow l y = zip [(x, y) | x <- [0..]] l
        arr = array ((0, 0), (19, 19)) $ concat $ zipWith arrRow s' [0..]
    in maximum $ map (productOfLine arr) $ arrayLines 20 20 4 

euler011 :: IO String
euler011 = show . compute <$> getInput 11