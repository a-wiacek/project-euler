module Solutions.Euler088 where
import Utils.List(uniques)

-- productSum k \in [k, 2k]
-- since each ai >= 1 and example product sum is a1 = k, a2 = 2, a3 = .. = a_k = 1

check :: Int -> Int -> Int -> Int -> Bool
check remLen remProd remSum maxN
    | remLen == 0 = remProd == 1 && remSum == 0
    | remProd < 1 = False
    | remProd == 1 = remSum == remLen -- fill remaining holes with 1
    | remProd > 1 = any (\x -> check (remLen - 1) (remProd `div` x) (remSum - x) x)
                       $ filter ((==0) . mod remProd) [2..maxN]

iter :: Int -> Int -> Int
iter n k = if check k n n k then n else iter (n + 1) k

productSum :: Int -> Int
productSum k = iter k k

solution :: Int -> Int
solution n = sum $ uniques $ map productSum [2..n]

euler088 :: IO String
euler088 = return $ show $ solution 12000