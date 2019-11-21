module Solutions.Euler121 where

pSingleRed = map ((1-) . (1/)) [2..] :: [Double]

blue = (1-)
red = id
chooseRed' 0 n = [replicate n blue]
chooseRed' _ 0 = [[]]
chooseRed' k n = map (red:) (chooseRed' (k - 1) (n - 1)) ++ map (blue:) (chooseRed' k (n - 1))
chooseRed n = chooseRed' ((n - 1) `div` 2) n

ans n = floor $ (1/) $ sum [product $ zipWith ($) l $ take n pSingleRed | l <- chooseRed n]

euler121 :: IO String
euler121 = return $ show $ ans 15