module Solutions.Euler181 where
import Control.Monad
import Control.Monad.ST
import Data.Array.ST

partitions bMax wMax = runST $ do
    arr <- newArray ((0, 0), (bMax, wMax)) 0 :: ST s (STArray s (Int, Int) Int)
    writeArray arr (0, 0) 1
    forM_ [0..bMax] $ \black -> forM_ [0..wMax] $ \white -> when (black + white > 0) $
        forM_ [black..bMax] $ \i -> forM_ [white..wMax] $ \j -> do
            p <- readArray arr (i - black, j - white) 
            q <- readArray arr (i, j)
            writeArray arr (i, j) (p + q)
    readArray arr (bMax, wMax)

euler181 :: IO String
euler181 = return $ show $ partitions 60 40