module Solutions.Euler187 where
import Utils.NumberTheory(unPrime, primes)
import Utils.Operators((<&&>))

ps = map unPrime primes :: [Int]

bound = 10^8
f a = length $ takeWhile ((<=a) <&&> (<bound) . (*a)) ps

euler187 :: IO String
euler187 = return $ show $ sum $ map f $ takeWhile (<bound `div` 2) ps