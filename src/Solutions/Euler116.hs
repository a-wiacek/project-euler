module Solutions.Euler116 where
import Data.Array.Unboxed
import Data.Array.ST
import Control.Monad

generator :: Int -> UArray Int Int
generator k = runSTUArray $ do
    arr <- newArray_ (0, 50)
    forM_ [0..k - 1] $ \x -> writeArray arr x 1
    forM_ [k..50] $ \x -> (+) <$> readArray arr (x - 1) <*> readArray arr (x - k) >>= writeArray arr x
    mapArray pred arr -- remove option with grey tiles only

red = generator 2
green = generator 3
blue = generator 4

euler116 :: IO String
euler116 = return $ show $ red ! 50 + blue ! 50 + green ! 50