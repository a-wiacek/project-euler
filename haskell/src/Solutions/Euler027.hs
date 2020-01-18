module Solutions.Euler027 where
import Utils.NumberTheory(primesArrayUpTo)
import Data.Array.Unboxed

primes :: UArray Int Bool
primes = primesArrayUpTo 2000000

compute :: Int -> Int -> Int
compute a' b' =
    let comp' a b n =
            let n' = n * n + a * n + b
            in if n' >= 2 && primes ! n'
                then comp' a b (n + 1)
                else n
    in comp' a' b' 0

max' :: (Int, Int) -> (Int, Int) -> (Int, Int)
max' (a, b) (c, d) = if a > c then (a, b) else (c, d)

euler027 :: IO String
euler027 = return $ show $ snd $ foldr1 max' [(compute a b, a * b) | a <- [-999..999], b <- [-1000..1000]]