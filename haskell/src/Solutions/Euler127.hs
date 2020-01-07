module Solutions.Euler127 where
import Utils.Array(modifyArray)
import Utils.NumberTheory(unPrime, primesUpTo)
import Control.Monad
import Data.Array.ST
import Data.Array.Unboxed

bound = 120000 :: Int
radicalsArray = runSTUArray $ do
    arr <- newArray (1, bound) 1
    forM_ (map unPrime $ primesUpTo bound) $ \p ->
        forM_ (takeWhile (<=bound) [p, 2 * p..]) $ \x ->
            modifyArray arr x (*p)
    return arr
rad a b c = radicalsArray ! a * radicalsArray ! b * radicalsArray ! c

-- since a, b and c are coprime, rad a b c = rad a * rad b * rad c

hits c = sum [c | a <- [1..(c - 1) `div` 2], let b = c - a, gcd a b == 1, rad a b c < c]

euler127 :: IO String
euler127 = return $ show $ sum $ map hits [1..bound - 1]