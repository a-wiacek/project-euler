module Solutions.Euler201 where
import Utils.Array(modifyArray)
import Control.Monad
import Control.Monad.ST
import Data.Array.ST

compute :: Int -> [Int] -> Int
compute u l =
    let s = sum l
        n = length l
    in runST $ do
        arr <- newArray ((0, 0), (s, n)) 0 :: ST s (STUArray s (Int, Int) Int)
        writeArray arr (0, 0) 1
        forM_ l $ \el -> forM_ [s, s - 1..el] $ \x -> forM_ [n, n - 1..1] $ \y ->
            readArray arr (x - el, y - 1) >>= modifyArray arr (x, y) . (+)
        sum <$> forM [0..s] (\x -> readArray arr (x, u) >>= \v -> return (if v == 1 then x else 0))

euler201 :: IO String
euler201 = return $ show $ compute 50 $ map (^2) [1..100]