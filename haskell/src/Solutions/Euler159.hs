module Solutions.Euler159 where
import qualified Data.Set as Set
import Data.Array
import Utils.Array(funArray)
import Utils.Numeric(digitalRoot)

sumMdrs :: Int -> Int
sumMdrs bound =
    let mdrsArr :: Array Int Int
        mdrsArr = funArray 2 (bound - 1) mdrsFun

        mdrsFun :: Int -> Int
        mdrsFun n = foldr f (digitalRoot n) lsq
            where lsq = filter (\x -> n `mod` x == 0) $ takeWhile (\x -> x * x <= n) [2..]
                  f y x = max x (mdrsArr ! y + mdrsArr ! (n `div` y))
    in sum $ elems mdrsArr

euler159 :: IO String
euler159 = return $ show $ sumMdrs 1000000