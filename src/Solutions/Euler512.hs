module Solutions.Euler512 where
import Control.Monad
import Control.Monad.ST
import Data.Array.Unboxed
import Data.Array.ST
import Data.STRef
import Data.Word
import Utils.Array(modifyArray)

plus :: Word32 -> Int -> Int
plus a b = fromIntegral a + b

-- Copied totientArrayUpTo from NumberTheory just to change type
funG :: Word32 -> Int
funG n = runST $ do
    arr <- newListArray (1, n) [1..] :: ST s (STUArray s Word32 Word32)
    forM_ [2..n] $ \x -> readArray arr x >>= \val -> when (val == x) $ do
        writeArray arr x (x - 1)
        forM_ [2 * x, 3 * x..n] $ \y ->
            modifyArray arr y ((*(x - 1)) . (`div` x))
    total <- newSTRef (0 :: Int)
    forM_ [1, 3..n] $ readArray arr >=> modifySTRef' total . plus
    readSTRef total

euler512 :: IO String
euler512 = return $ show $ funG 500000000