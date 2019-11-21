module Solutions.Euler475 where
import Utils.Memo
import Utils.Numeric(fastPowerMod, binom)
import Control.Monad
import qualified Data.Map.Strict as Map

-- TOFIX: Too much memory taken

pp = 10^9 + 7 :: Int
p = toInteger pp
type Key = (Int, Int, Int, Int)

base = 151
keyToInt :: Key -> Int
keyToInt (a, b, c, d) = a + base * b + base^2 * c + base^3 * d
intToKey :: Int -> Key
intToKey n = let a = n `mod` base
                 b = n `mod` base^2 `div` base
                 c = n `mod` base^3 `div` base^2
                 d = n `mod` base^4 `div` base^3
             in (a, b, c, d)

split3 :: [Key]
split3 = [(a, b, c, 3 - a - b - c) | a <- [0..3], b <- [0..3 - a], c <- [0..3 - a - b]]

less :: Key -> Key -> Bool
less (a, b, c, d) (w, x, y, z) = and [a <= w, b <= x, c <= y, d <= z]

binom' :: Int -> Int -> Integer
binom' a b = binom (toInteger a) (toInteger b)

reduce :: Integer -> Int
reduce = fromIntegral . flip mod p

mulmod :: Int -> Int -> Int
mulmod a b = a * b `mod` pp

modfmap :: Functor f => (a -> Int) -> f a -> f Int
modfmap f a = (`mod` pp) . f <$> a 

compute :: Int -> StrictMemo Int Int Int
compute arg' =
    let arg@(n1, n2, n3, n4) = intToKey arg'
    in modfmap sum $ forM split3 $ \split@(k1, k2, k3, k4) -> if less split arg
        then let s = foldr1 mulmod $ map reduce [binom' n1 k1, binom' n2 k2 * 2^k2, binom' n3 k3 * 3^k3, binom' n4 k4 * 4^k4]
             in modfmap (*s) $ memo compute (keyToInt (n1 - k1 + k2, n2 - k2 + k3, n3 - k3 + k4, n4 - k4))
        else return 0

-- By Little Fermat's theorem a^(-1) = a^(p - 2) mod p
solutions n =
    let solutions' = toInteger $ evalMemo (compute $ keyToInt (0, 0, 0, fromIntegral n `div` 4)) $ Map.singleton 0 1
    in mod (solutions' * fastPowerMod (product [1..n `div` 3]) (p - 2) p) p

euler475 :: IO String
euler475 = return $ show $ solutions 600