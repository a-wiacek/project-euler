module Solutions.Euler191 where
import Control.Monad
import Control.Monad.ST
import Data.Array.ST

count = runST $ do
    -- length of word, 'L's used, 'A's on end
    arr <- newArray ((0, 0, 0), (200, 1, 2)) 0 :: ST s (STArray s (Int, Int, Int) Integer)
    let writeArr = writeArray arr
    let readArr = readArray arr
    writeArr (0, 0, 0) 1
    forM_ [1..30] $ \x -> do
        p00 <- readArr (x - 1, 0, 0)
        p01 <- readArr (x - 1, 0, 1)
        p02 <- readArr (x - 1, 0, 2)
        p10 <- readArr (x - 1, 1, 0)
        p11 <- readArr (x - 1, 1, 1)
        p12 <- readArr (x - 1, 1, 2)
        writeArr (x, 0, 0) $ p00 + p01 + p02
        writeArr (x, 0, 1) p00
        writeArr (x, 0, 2) p01
        writeArr (x, 1, 0) $ p00 + p01 + p02 + p10 + p11 + p12
        writeArr (x, 1, 1) p10
        writeArr (x, 1, 2) p11
    r00 <- readArr (30, 0, 0)
    r01 <- readArr (30, 0, 1)
    r02 <- readArr (30, 0, 2)
    r10 <- readArr (30, 1, 0)
    r11 <- readArr (30, 1, 1)
    r12 <- readArr (30, 1, 2)
    return $ r00 + r01 + r02 + r10 + r11 + r12

euler191 :: IO String
euler191 = return $ show count