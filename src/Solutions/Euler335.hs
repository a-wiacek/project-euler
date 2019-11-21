module Solutions.Euler335 where
import Utils.NumberTheory(invertMod)
import Utils.Numeric(fastPowerMod)

-- Values for small sums found using simulation: [2, 5, 15, 53, 207, 845, 3495, 14453, 59487, 243485, 991575]
-- Explicit formula found using a bit of guesswork: M(2^n + 1) = 2 * 2^n - 3^n + 4^n
-- \sum_{i = 0}^n M(2^i + 1) = 2 * (2^(n + 1) - 1) - (3^(n + 1) - 1) / 2 + (4^(n + 1) - 1) / 3

m = 7^9

sumM :: Integer -> Integer
sumM n =
    let f x = fastPowerMod x (n + 1) m
        Just inv2 = invertMod 2 m
        Just inv3 = invertMod 3 m
    in (2 * (f 2 - 1) - (f 3 - 1) * inv2 +  (f 4 - 1) * inv3) `mod` m

euler335 :: IO String
euler335 = return $ show $ sumM $ 10^18