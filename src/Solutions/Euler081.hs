module Solutions.Euler081 where
import Utils.Input(getInput)
import Control.Monad
import Control.Monad.ST
import Data.Array
import Data.Array.ST

type Matrix = Array (Int, Int) Int

ixs :: Int -> [(Int, Int)] -- this is intentionally transposed
ixs n = [(y, x) | x <- [0..n - 1], y <- [0..n - 1]]

parseRow :: String -> [Int]
parseRow s = read $ "[" ++ s ++ "]"

parseMatrix :: String -> Matrix
parseMatrix s =
    let rows = map parseRow $ lines s
        n = length rows
        associates = zip (ixs n) (concat rows)
    in array ((0, 0), (n - 1, n - 1)) associates

findMinPath :: Matrix -> Int
findMinPath matrix = let n = snd $ snd $ bounds matrix in runST $ do
    arr <- newArray ((0, 0), (n, n)) 0 :: ST s (STArray s (Int, Int) Int)
    writeArray arr (0, 0) $ matrix ! (0, 0)
    forM_ [1..n] $ \x -> do
        readArray arr (x - 1, 0) >>= writeArray arr (x, 0) . (+(matrix ! (x, 0)))
        readArray arr (0, x - 1) >>= writeArray arr (0, x) . (+(matrix ! (0, x)))
    forM_ [1..n] $ \x -> forM_ [1..n] $ \y -> do
        v1 <- readArray arr (x - 1, y)
        v2 <- readArray arr (x, y - 1)
        writeArray arr (x, y) $ matrix ! (x, y) + min v1 v2
    readArray arr (n, n)

euler081 :: IO String
euler081 = show . findMinPath . parseMatrix <$> getInput 81