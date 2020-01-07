module Solutions.Euler347 where
import Utils.List(uniques)
import Utils.NumberTheory(unPrime, primes)

ps = map unPrime primes :: [Int]

funM :: Int -> Int -> Int -> Int
funM p q n =
    let list :: Int -> [Int]
        list r | q * r > n = if r `mod` p == 0 then r : list (r `div` p) else []
               | otherwise = list $ q * r
        initList = takeWhile (<=n) $ map ((*q) . (p^)) [1..]
    in if null initList then 0 else maximum $ list $ last initList

funS :: Int -> Int
funS n =
    let allps = takeWhile ((<=n) . (^2)) ps
        qs p = takeWhile (\q -> p * q <= n) $ dropWhile (<=p) ps
        ms p = map (\q -> funM p q n) $ qs p
    in sum $ uniques $ concatMap ms allps

euler347 :: IO String
euler347 = return $ show $ funS 10000000