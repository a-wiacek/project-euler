module Solutions.Euler149 where
import Utils.Array(modifyArray)
import Control.Monad
import Control.Monad.ST
import Data.Array.Unboxed
import Data.Array.ST

bound = 2000 :: Int

column x = [(x, y) | y <- [1..bound]]
row y = [(x, y) | x <- [1..bound]]
diagonal1 initY = [(t, t') | t <- [1..bound], let t' = initY - t, t' <= bound, t' > 0] -- LU to RD
diagonal2 initY = [(t, t') | t <- [1..bound], let t' = initY + t, t' <= bound, t' > 0] -- LD to RU
allLines = [column x | x <- [1..bound]] ++ [row y | y <- [1..bound]] ++
        [diagonal1 y | y <- [2..2 * bound]] ++ [diagonal2 y | y <- [-bound + 1..bound - 1]]

maxSublistSum l = let n = length l in maximum $ elems $ runSTArray $ do
    arr <- newListArray (0, n - 1) l
    forM_ [1..n - 1] $ \i -> readArray arr (i - 1) >>= \e ->
        when (e > 0) $ modifyArray arr i (+e)
    return arr

f :: Int -> Int
f x = x `mod` 1000000 - 500000

compute :: Int
compute = runST $ do
    linearS <- newArray (1, bound * bound) 0 :: ST s (STUArray s Int Int)
    forM_ [1..55] $ \k -> writeArray linearS k $ f $ 100003 - 200003 * k + 300007 * (k^3)
    forM_ [56..bound * bound] $ \k ->
        (+) <$> readArray linearS (k - 24) <*> readArray linearS (k - 55) >>= writeArray linearS k . f
    elems <- getElems linearS
    squareS <- newListArray ((1, 1), (bound, bound)) elems :: ST s (STUArray s (Int, Int) Int)
    maximum <$> forM allLines (\l -> maxSublistSum <$> forM l (readArray squareS))

euler149 :: IO String
euler149 = return $ show compute 