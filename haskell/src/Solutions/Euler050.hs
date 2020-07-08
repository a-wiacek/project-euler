module Solutions.Euler050 where
import Utils.List(maxBy)
import Utils.NumberTheory(primesArrayUpTo)
import Data.Array.Unboxed

primes = primesArrayUpTo 1000000
prefixSums = takeWhile (<2000000) $ scanl (+) 0 $ map fst . filter snd . assocs $ primes

compute =
    let l = length prefixSums
        arr = listArray (1, l) prefixSums :: UArray Int Int
    in snd $ maxBy fst [(b - a, p) |
        a <- [1..l],
        b <- [a + 22..l],
        let p = arr ! b - arr ! a,
        p < 1000000,
        primes ! p]

euler050 :: IO String
euler050 = return $ show compute