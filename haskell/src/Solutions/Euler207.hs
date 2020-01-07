module Solutions.Euler207 where
import Control.Monad
import Control.Monad.ST
import Data.STRef

{-
Let x = 2^t, then we solve equation x^2 = x + k -> x^2 - x - k = 0
We are interested only in positive solution, so x = (1 + sqrt (1 + 4k)) / 2,
so k = ((2x - 1)^2 - 1)/4. If x is integer, k is also integer, since square of odd number is equal to 1 modulo 4.
-}

isPower2 1 = True
isPower2 n = even n && isPower2 (n `div` 2)

compute :: Int
compute = runST $ do
    perfectP <- newSTRef 0
    totalP <- newSTRef 1 -- x = totalP + 1
    let loop = do x <- succ <$> readSTRef totalP
                  when (isPower2 x) $ modifySTRef' perfectP succ
                  modifySTRef' totalP succ
                  t <- readSTRef totalP
                  p <- readSTRef perfectP
                  when (12345 * p >= t) loop
    loop
    lx <- readSTRef totalP
    return $ ((2 * lx + 1)^2 - 1) `div` 4

euler207 :: IO String
euler207 = return $ show compute 