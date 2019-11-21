module Solutions.Euler077 where
import Utils.NumberTheory(unPrime, primes)

ps = map unPrime primes :: [Int]
primeSum n (p:ps)
    | n == 0 = 1
    | n < p = 0
    | otherwise = primeSum (n - p) (p:ps) + primeSum n ps

euler077 :: IO String
euler077 = return $ show $ head $ dropWhile (\x -> primeSum x ps <= 5000) [2..]