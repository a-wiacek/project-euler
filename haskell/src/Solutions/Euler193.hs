module Solutions.Euler193 where
import Utils.NumberTheory(unPrime, primesUpTo)

-- Inclusion–exclusion principle

compute :: Integer -> Integer
compute bound2 =
    let bound = floor $ sqrt $ fromIntegral bound2
        getNumbers :: Integer -> [Integer] -> Integer
        getNumbers prod [] = bound2 `quot` prod
        getNumbers prod (p:ps)
            | abs (p * prod) >= bound2 = bound2 `quot` prod
            | p >= bound2 = 0
            | otherwise = getNumbers prod ps + getNumbers ((-p) * prod) ps
    in getNumbers 1 $ map ((^2) . unPrime) $ primesUpTo bound

euler193 :: IO String
euler193 = return $ show $ compute $ 2^50