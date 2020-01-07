module Solutions.Euler174 where
import Utils.Array(modifyArray)
import Utils.Operators((<&&>))
import Control.Monad
import Control.Monad.ST
import Data.Array.ST
import Data.Array.Unboxed

laminaeWays bound = runSTUArray $ do
    arr <- newArray (1, bound) 0 :: ST s (STUArray s Int Int)
    forM_ [3..2500001] $ \l ->
        forM_ (takeWhile (\t -> 4 * t * (l - t) <= bound) [1..(l - 1) `div` 2]) $ \t ->
            let index = 4 * t * (l - t) in modifyArray arr index succ
    return arr

euler174 :: IO String
euler174 = return $ show $ length $ filter ((>=1) <&&> (<=10)) $ elems $ laminaeWays (10^6)