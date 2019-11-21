module Solutions.Euler321 where
import Utils.List(interweave)

-- http://oeis.org/A005563

-- 2(n + 1)^2 - 2 = k^2 + k => 2n^2 + 4n - k^2 - k = 0
-- https://www.alpertron.com.ar/QUAD.HTM

f (x, y) = (3 * x + 2 * y + 3, 4 * x + 3 * y + 5)
g = tail . iterate f

triangleMs :: [Integer]
triangleMs = map fst $ interweave (g (0, -1)) (g (0, 0))

euler321 :: IO String
euler321 = return $ show $ sum $ take 40 triangleMs