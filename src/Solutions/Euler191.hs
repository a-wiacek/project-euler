module Solutions.Euler191 where
import Control.Monad
import Control.Monad.ST
import Data.Array.ST

layer k = [(k, u, v) | u <- [0..1], v <- [0..2]]

count = runST $ do
    -- length of word, 'L's used, 'A's on end
    arr <- newArray ((0, 0, 0), (200, 1, 2)) 0 :: ST s (STArray s (Int, Int, Int) Integer)
    writeArray arr (0, 0, 0) 1
    forM_ [1..30] $ \x -> do
        ps@[p00, p01, p02, p10, p11, p12] <- mapM (readArray arr) (layer $ x - 1)
        mapM (uncurry $ writeArray arr)
            [ ((x, 0, 0), p00 + p01 + p02)
            , ((x, 0, 1), p00)
            , ((x, 0, 2), p01)
            , ((x, 1, 0), sum ps)
            , ((x, 1, 1), p10)
            , ((x, 1, 2), p11)
            ]
    sum <$> mapM (readArray arr) (layer 30)

euler191 :: IO String
euler191 = return $ show count