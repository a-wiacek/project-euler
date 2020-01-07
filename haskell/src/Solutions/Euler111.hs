module Solutions.Euler111 where
import Utils.List(combinations2)
import Control.Monad(replicateM)
import Data.List
import Math.NumberTheory.Primes.Testing(isPrime)

digitsToInt :: [Int] -> Int
digitsToInt = foldl' (\x y -> 10 * x + y) 0

repeatable :: Int -> Int -> Int -> [Int]
repeatable n d dTimes =
    let digitsToChoose = delete d [0..9]
    in [digitsToInt digits |
        (dPositions, remainingPositions) <- combinations2 dTimes [0..n - 1],
        remainingDs <- replicateM (n - dTimes) digitsToChoose,
        let digits = map snd $ sort $ zip dPositions (repeat d) ++ zip remainingPositions remainingDs,
        head digits /= 0]

repeatablePrimes :: Int -> Int -> Int -> [Int]
repeatablePrimes n d dTimes = filter (isPrime . toInteger) $ repeatable n d dTimes

funS :: Int -> Int -> Int
funS n d = sum $ head $ filter (not . null) $ map (repeatablePrimes n d) [n, n - 1..]

sumFunS :: Int -> Int
sumFunS n = sum $ map (funS n) [0..9]

euler111 :: IO String
euler111 = return $ show $ sumFunS 10