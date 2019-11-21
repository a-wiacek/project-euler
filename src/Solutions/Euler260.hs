module Solutions.Euler260 where
import Utils.Operators((<&&>))
import Control.Monad
import Data.Array.ST
import Data.Array.Unboxed

type I = (Int, Int, Int)

isSorted :: I -> Bool
isSorted (a, b, c) = a >= b && b >= c
sumI :: I -> Int
sumI (a, b, c) = a + b + c

sort :: I -> I
sort (a, b, c)
    | a < b =
        if b < c
            then (c, b, a)
            else if a < c
                then (b, c, a)
                else (b, a, c)
    | b < c =
            if a < c
                then (c, a, b)
                else (a, c, b)
    | otherwise = (a, b, c)

run :: Int -> UArray I Bool
run n = runSTUArray $ do
    arr <- newArray ((0, 0, 0), (n, n, n)) False -- True is winning position, False is losing
    forM_ [0..n] $ \x -> forM_ [0..x] $ \y -> forM_ [0..y] $ \z -> -- x >= y >= z
        readArray arr (x, y, z) >>= \p -> unless p $ do
            forM_ [1..n - x] $ \d -> writeArray arr (x + d, y, z) True
            forM_ [1..n - y] $ \d -> writeArray arr (sort (x, y + d, z)) True
            forM_ [1..n - z] $ \d -> writeArray arr (sort (x, y, z + d)) True
            forM_ [1..n - x] $ \d -> writeArray arr (x + d, y + d, z) True
            forM_ [1..n - x] $ \d -> writeArray arr (sort (x + d, y, z + d)) True
            forM_ [1..n - y] $ \d -> writeArray arr (sort (x, y + d, z + d)) True
            forM_ [1..n - x] $ \d -> writeArray arr (x + d, y + d, z + d) True
    return arr

f = sum . map (sumI . fst) . filter (not . snd <&&> isSorted . fst) . assocs . run

euler260 :: IO String
euler260 = return $ show $ f 1000