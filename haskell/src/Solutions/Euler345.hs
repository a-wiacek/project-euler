module Solutions.Euler345 where
import Utils.Input(getInput)
import Control.Monad
import Control.Monad.ST
import Data.Array.Unboxed
import Data.Bits
import qualified Data.IntMap.Strict as IntMap
import Data.STRef

matrixSum :: UArray (Int, Int) Int -> Int
matrixSum matrix =
    let n = snd $ snd $ bounds matrix
    in runST $ do cache <- newSTRef $ IntMap.singleton 1 0
                  forM_ [1..n] $ \r -> do
                      stepCache <- newSTRef IntMap.empty
                      forM_ [1..n] $ \c -> let x = 2^c in 
                          IntMap.toList <$> readSTRef cache >>= mapM_ (\(key, value) ->
                              when (x .&. key == 0) $ modifySTRef' stepCache $ IntMap.insertWith max (x .|. key) (matrix ! (c, r) + value))
                      readSTRef stepCache >>= writeSTRef cache
                  (IntMap.! (2^(n + 1) - 1)) <$> readSTRef cache

euler345 :: IO String
euler345 = show . matrixSum . listArray ((1, 1), (15, 15)) . map read . words <$> getInput 345