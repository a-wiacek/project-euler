module Solutions.Euler662 where
import Control.Monad
import Control.Monad.ST
import Data.Array.Unboxed
import Data.Array.ST
import Data.Word
import Utils.Array(modifyArray)
import Utils.Numeric(fibonacci)

type W = Word32

squares :: W -> [(W, W)]
squares n = [(x, y) | x <- [0..n], y <- [0..n], x^2 + y^2 == n^2]

m = 1000000007 :: W

plus :: W -> W -> W
plus a b = let s = a + b in if s > m then s - m else s

funF :: W -> W -> W
funF width height =
    let paths = concatMap squares $ takeWhile (<width + height) $ fibonacci 1 2
    in runST $ do
        arr <- newArray ((0, 0), (width, height)) 0 :: ST s (STUArray s (W, W) W)
        writeArray arr (0, 0) 1
        forM_ paths $ \(dx, dy) -> forM_ [0..width] $ \x -> forM_ [0..height] $ \y ->
            when (x >= dx && y >= dy) $ readArray arr (x - dx, y - dy) >>= modifyArray arr (x, y) . plus
        readArray arr (width, height)

euler662 :: IO String
euler662 = return $ show $ funF 10000 10000