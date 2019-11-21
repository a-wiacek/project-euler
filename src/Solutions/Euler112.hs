module Solutions.Euler112 where
import Utils.Operators((<||>))
import Control.Monad
import Control.Monad.ST
import Data.STRef

inc :: String -> Bool
inc [c] = True
inc (h1:h2:t) = h1 <= h2 && inc (h2:t)
dec :: String -> Bool
dec = inc . reverse
isBouncy :: Int -> Bool
isBouncy = not . (inc <||> dec) . show

compute = runST $ do
    n <- newSTRef 99
    bouncy <- newSTRef 0
    let loop = do
            n' <- readSTRef n
            bouncy' <- readSTRef bouncy
            when (100 * bouncy' < 99 * n') $ do
                when (isBouncy $ n' + 1) $ modifySTRef bouncy succ
                modifySTRef n succ
                loop
    loop
    readSTRef n

euler112 :: IO String
euler112 = return $ show compute