module Solutions.Euler164 where
import Utils.Array(modifyArray)
import Control.Monad
import Control.Monad.ST
import Data.Array.ST

-- (length left, first two digits)
type C = (Int, Int, Int)

compute :: Int
compute = runST $ do
    arr <- newArray ((2, 0, 0), (20, 9, 9)) 0 :: ST s (STArray s C Int)
    forM_ [0..9] $ \i -> forM_ [0..9] $ \j -> when (i + j <= 9) (writeArray arr (2, i, j) 1)
    forM_ [3..20] $ \s -> forM_ [0..9] $ \i -> forM_ [0..9 - i] $ \j -> do
        plus <- readArray arr (s - 1, i, j)
        forM_ [0..9 - i - j] $ \d ->
            modifyArray arr (s, d, i) (+plus)
    sum <$> forM [1..9] (\i -> sum <$> forM [0..9] (\j -> readArray arr (20, i, j)))

euler164 :: IO String
euler164 = return $ show compute