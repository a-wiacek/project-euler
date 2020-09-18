{-# LANGUAGE FlexibleContexts #-}
module Solutions.Euler196 where
import Control.Monad
import Data.Array.ST
import Data.Array.Unboxed
import Math.NumberTheory.Primes.Testing(isPrime)

type A = UArray (Int, Int)

generateArray :: Int -> (A Int, A Bool)
generateArray n =
    let row1 = [(n - 2) * (n - 3) `div` 2 + 1..(n - 1) * (n - 2) `div` 2] ++ [4, 4, 4, 4]
        row2 = [(n - 1) * (n - 2) `div` 2 + 1..n * (n - 1) `div` 2]       ++ [4, 4, 4]
        row3 = [(n - 1) * n `div` 2 + 1..n * (n + 1) `div` 2]             ++ [4, 4]
        row4 = [n * (n + 1) `div` 2 + 1..(n + 1) * (n + 2) `div` 2]       ++ [4]
        row5 = [(n + 1) * (n + 2) `div` 2 + 1..(n + 2) * (n + 3) `div` 2]
        rows = row1 ++ row2 ++ row3 ++ row4 ++ row5
        mkArr :: IArray UArray a => [a] -> A a
        mkArr = listArray ((1, 1), (5, n + 2))
    in (mkArr rows, mkArr $ map (isPrime . toInteger) rows)

neighbours :: (Int, Int) -> Int -> [(Int, Int)]
neighbours (x, y) maxY = [(x', y') | x' <- [x - 1..x + 1], y' <- [y - 1..y + 1],
                                    (x', y') /= (x, y),
                                    x' > 0, x' <= 5,
                                    y' > 0, y' <= maxY]
    
markTriplets :: A Bool -> A Bool
markTriplets initArray = let (_, (_, maxY)) = bounds initArray in runSTUArray $ do
    arr <- newArray (bounds initArray) False
    forM_ [1..5] $ \x -> forM_ [1..maxY] $ \y -> when (initArray ! (x, y)) $
        let nbs = neighbours (x, y) maxY
        in filter id <$> forM nbs (return . (initArray !)) >>= \ps ->
           when (length ps >= 2) $ forM_ ((x, y):nbs) $ \pos -> writeArray arr pos (initArray ! pos)
    return arr

countRow :: Int -> Int
countRow n =
    let (arrInt, arrIsPrime) = generateArray n
        arrIsMarked = markTriplets arrIsPrime
        f k = if arrIsMarked ! (3, k) then arrInt ! (3, k) else 0
    in sum $ map f [1..n + 2]

euler196 :: IO String
euler196 = return $ show $ countRow 5678027 + countRow 7208785