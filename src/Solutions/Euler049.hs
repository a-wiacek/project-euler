module Solutions.Euler049 where
import Utils.List(uniques)
import Utils.NumberTheory(primesArrayUpTo)
import Data.Array.Unboxed((!))
import Data.List

primes = primesArrayUpTo 10000

rearrange :: Int -> Int
rearrange = read . sort . show

properSeq :: [Int] -> Bool
properSeq l
    | l == [1487, 4817, 8147] = False
    | not $ all (primes !) l = False
    | otherwise = (1==) $ length $ uniques $ map rearrange l

findTriplet = head [concatMap show l |
    a <- [1000..9999],
    b <- [1..(10000 - a) `div` 2],
    let l = [a, a + b, a + b + b],
    properSeq l]

euler049 :: IO String
euler049 = return findTriplet