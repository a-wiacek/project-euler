module Solutions.Euler028 where
import Control.Monad
import Control.Monad.ST
import Data.STRef

compute = runST $ do
    sum <- newSTRef 1 
    step <- newSTRef 2
    curr <- newSTRef 1
    replicateM_ 500 $ do
        replicateM_ 4 $ do
            readSTRef step >>= modifySTRef curr . (+)
            readSTRef curr >>= modifySTRef sum . (+)
        modifySTRef step (+2)
    readSTRef sum

euler028 :: IO String
euler028 = return $ show compute