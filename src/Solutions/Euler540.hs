module Solutions.Euler540 where
import Utils.NumberTheory(unPrime, primesUpTo)

-- Details of algorithm: http://vixra.org/pdf/1310.0211v1.pdf
-- Algorithm in pdf is counting triples one by one. Since answer is very close
-- to 10^15, we can't count them one by one. We need to do it faster.
-- https://math.stackexchange.com/questions/218890/how-many-numbers-in-a-given-range-are-coprime-to-n
-- http://emis.impa.br/EMIS/journals/INTEGERS/papers/i41/i41.pdf (theorem 3b for k = 1)
bound, bound' :: Int
bound = 3141592653589793
bound' = floor $ sqrt $ fromIntegral bound * 0.6

primes = map unPrime $ primesUpTo bound'
uberdiv n p = if n `mod` p == 0 then uberdiv (n `div` p) p else n
factors :: Int -> [Int]
factors n = f n primes [] where
    f n (p:ps) acc | n == 1 = acc
                   | p * p > n = n:acc
                   | n `mod` p == 0 = f (n `uberdiv` p) ps (p:acc)
                   | otherwise = f n ps acc

powerMobius :: [Int] -> [Int]
powerMobius [] = [1]
powerMobius (h:t) = let t' = powerMobius t in map (*(-h)) t' ++ t'

coprime :: Int -> Int -> Int
coprime n b = sum $ map (quot b) $ powerMobius $ factors n

case1 :: Int -> Int
case1 n' =
    let n = fromIntegral n' :: Double
    in sum [coprime i' nv - coprime i' n1 |
                i' <- [1, 3..floor $ sqrt $ n / (2 + sqrt 2)],
                let i = fromIntegral i' :: Double,
                let n1 = floor (i / sqrt 2) :: Int,
                let nv = floor $ (sqrt (n + n - i * i) - i) / 2 :: Int]

case2 :: Int -> Int
case2 n' =
    let n = fromIntegral n' :: Double
    in sum [coprime (i' + i') nv - coprime (i' + i') n1 |
                i' <- [1..floor $ sqrt $ n / (4 + sqrt 8)],
                let i = fromIntegral i' :: Double,
                let n1 = floor (i * sqrt 2) :: Int,
                let nv = floor $ sqrt (n - i * i) - i :: Int]
    -- 2 * i' trick allows us to exclude all even numbers

pythagorean n = case1 n + case2 n

euler540 :: IO String
euler540 = return $ show $ pythagorean bound