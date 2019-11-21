module Simulations.Euler335 where
import Control.Monad
import Control.Monad.ST
import Data.Array.Unboxed
import Data.Array.ST
import Data.STRef
import Utils.Array

simulate :: Int -> Int
simulate n = runST $ do
    arr <- newArray (1, n) 1 :: ST s (STUArray s Int Int)
    index <- newSTRef 1
    let pp x = if x == n then 1 else x + 1
        check = all (==1) <$> getElems arr
        putStone = do modifySTRef' index pp
                      i <- readSTRef index
                      modifyArray arr i succ
        step = do i <- readSTRef index
                  stones <- readArray arr i
                  writeArray arr i 0
                  replicateM_ stones putStone
        loop n = step >> check >>= \p -> if p then return n else loop (n + 1)
    loop 1

runSimulation335 :: IO ()
runSimulation335 = print $ map (\x -> let u = 2^x + 1 in simulate u) [0..10]