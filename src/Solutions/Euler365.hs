module Solutions.Euler365 where
import Utils.NumberTheory(unPrime, nextPrime, precPrime)
import Utils.Numeric(binom)
import qualified Data.IntMap.Strict as IntMap
import Data.Maybe(fromJust)
import Math.NumberTheory.Moduli.Chinese(chineseRemainder)

primes' = map unPrime [nextPrime 1000 .. precPrime 5000]
binomMod1 :: Integer -> Integer -> Integer -> Integer -- (n choose k) mod p, by Lucas's theorem
binomMod1 n k p 
    | k == 0 || n == 0 = 1
    | otherwise = binom (n `mod` p) (k `mod` p) * binomMod1 (n `div` p) (k `div` p) p `mod` p

mod1p = IntMap.fromList $ zip primes' $ map (binomMod1 (10^18) (10^9) . toInteger) primes' :: IntMap.IntMap Integer
bimod :: Int -> Int -> Int -> Integer
bimod p q r =
    let get :: Int -> Integer
        get x = fromJust $ IntMap.lookup x mod1p
    in fromJust $ chineseRemainder [(get p, toInteger p), (get q, toInteger q), (get r, toInteger r)]

euler365 :: IO String
euler365 = return $ show $ sum [bimod p q r | p <- primes', q <- dropWhile (<=p) primes', r <- dropWhile (<=q) primes']