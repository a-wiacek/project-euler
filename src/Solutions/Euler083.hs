module Solutions.Euler083 where
import Utils.Input(getInput)
import Solutions.Euler081(Matrix, parseMatrix)
import Control.Monad
import Control.Monad.ST
import Data.Array
import Data.Array.ST
import Data.STRef

inf = 10^12
findMinPath :: Matrix -> Int
findMinPath matrix = let n = snd $ snd $ bounds matrix in runST $ do
    dist <- newArray ((0, 0), (n, n)) inf :: ST s (STArray s (Int, Int) Int)
    update <- newArray ((0, 0), (n, n)) False :: ST s (STArray s (Int, Int) Bool)
    writeArray dist (0, 0) $ matrix ! (0, 0)
    writeArray update (0, 0) True
    let selectIndex = do
            selected <- newSTRef ((-1, -1), inf)
            forM_ [0..n] $ \x -> forM_ [0..n] $ \y -> do
                wasUpdated <- readArray update (x, y)
                when wasUpdated $ do
                    (_, oldVal) <- readSTRef selected
                    newVal <- readArray dist (x, y)
                    when (newVal < oldVal) $ writeSTRef selected ((x, y), newVal)
            readSTRef selected
    let loop = do
            ((x, y), d) <- selectIndex
            when ((x, y) /= (-1, -1)) $ do
                writeArray update (x, y) False
                when (x > 0) $ do
                    let left = d + matrix ! (x - 1, y)
                    distLeft <- readArray dist (x - 1, y)
                    when (left < distLeft) $ do
                        writeArray dist (x - 1, y) left
                        writeArray update (x - 1, y) True
                when (x < n) $ do
                    let right = d + matrix ! (x + 1, y)
                    distRight <- readArray dist (x + 1, y)
                    when (right < distRight) $ do
                        writeArray dist (x + 1, y) right
                        writeArray update (x + 1, y) True
                when (y < n) $ do
                    let down = d + matrix ! (x, y + 1)
                    distDown <- readArray dist (x, y + 1)
                    when (down < distDown) $ do
                        writeArray dist (x, y + 1) down
                        writeArray update (x, y + 1) True
                when (y > 0) $ do
                    let up = d + matrix ! (x, y - 1)
                    distUp <- readArray dist (x, y - 1)
                    when (up < distUp) $ do
                        writeArray dist (x, y - 1) up
                        writeArray update (x, y - 1) True
                loop
    loop
    readArray dist (n, n)

euler083 :: IO String
euler083 = show . findMinPath . parseMatrix <$> getInput 83