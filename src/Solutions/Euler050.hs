module Solutions.Euler050 where
import Utils.NumberTheory(primesArrayUpTo)
import Data.Array.Unboxed

primes = primesArrayUpTo 1000000
prefixSums = takeWhile (<3000000) $ scanl (+) 0 $ map fst . filter snd . assocs $ primes

max' (a, b) (c, d) = if a > c then (a, b) else (c, d)

compute =
    let l = length prefixSums
        arr = listArray (1, l) prefixSums :: UArray Int Int
    in snd $ foldr1 max' [(b - a, p) |
        a <- [1..l],
        b <- [a + 22..l],
        let p = arr ! b - arr ! a,
        p < 1000000,
        primes ! p]

euler050 :: IO String
euler050 = return $ show compute