module Solutions.Euler134 where
import Utils.NumberTheory(primesUpTo, unPrime, invertMod)
import Data.Maybe(fromJust)

primes :: [Integer]
primes = map unPrime $ drop 2 $ primesUpTo 1000005

len n = let l' a acc = if a == 0 then acc else l' (a `div` 10) (acc + 1) in l' n 0
g = fromJust . invertMod 10
f p1 p2 = let l = len p1
              k = negate p1 * g p2 ^ l `mod` p2
          in 10^l * k + p1
ans (p1:p2:t) acc = ans (p2:t) (acc + f p1 p2)
ans [p] acc = acc

euler134 :: IO String
euler134 = return $ show $ ans primes 0