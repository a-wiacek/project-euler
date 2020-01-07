module Solutions.Euler139 where

-- https://en.wikipedia.org/wiki/Tree_of_primitive_Pythagorean_triples
matrixA (a, b, c) = (a - 2 * b + 2 * c, 2 * a - b + 2 * c, 2 * a - 2 * b + 3 * c)
matrixB (a, b, c) = (a + 2 * b + 2 * c, 2 * a + b + 2 * c, 2 * a + 2 * b + 3 * c)
matrixC (a, b, c) = (-a + 2 * b + 2 * c, -2 * a + b + 2 * c, -2 * a + 2 * b + 3 * c)

bound = 10^8 - 1
count t@(a, b, c)
    | per > bound = 0
    | otherwise = s + sum (map (count . ($t)) [matrixA, matrixB, matrixC])
    where per = a + b + c
          s = if c `mod` (b - a) == 0 then bound `div` per else 0

euler139 :: IO String
euler139 = return $ show $ count (3, 4, 5)