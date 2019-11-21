module Solutions.Euler618 where
import Data.Array
import Utils.NumberTheory(unPrime, primesUpTo)
import Utils.Numeric(fibonacci)

pMax = 46381 -- next prime after F_24
primes :: [Int]
primes = map unPrime $ primesUpTo pMax
primesArr :: Array Int Int
primesArr = listArray (1, length primes) primes

plus :: Int -> Int -> Int
plus a b = (a + b) `mod` 10^9

funS :: Array (Int, Int) Int
funS = array ((1, 0), (length primes, pMax)) [((x, y), f x y) | x <- [1..length primes], y <- [0..pMax]]

f :: Int -> Int -> Int
f pIndex sumLeft
    | sumLeft == p = p
    | sumLeft < p = 0
    | otherwise = plus (p * funS ! (pIndex, sumLeft - p)) (funS ! (pIndex + 1, sumLeft))
    where p = primesArr ! pIndex

euler618 :: IO String
euler618 = return $ show $ foldr plus 0 $ map (\k -> funS ! (1, k)) $ take 23 $ fibonacci 1 2