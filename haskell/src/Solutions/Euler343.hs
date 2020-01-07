module Solutions.Euler343 where
import Utils.Memo
import Utils.NumberTheory(isPrime, unPrime, primes)

ps :: [Int]
ps = map unPrime primes

-- Compute number of steps in algorithm.
compute :: Int -> StrictMemo Int Int Int
compute d
    | isPrime e = return d
    | otherwise = memo compute $ (\p -> e `div` p - 1) $ head $ filter (\x -> e `mod` x == 0) ps
    where e = d + 1

sumF :: Int -> StrictMemo Int Int Int
sumF n = sum <$> forM (map (^3) [1..n]) compute

euler343 :: IO String
euler343 = return $ show $ startEvalMemo $ sumF 2000000