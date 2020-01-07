module Solutions.Euler075 where
import Data.List

-- Primitive Pythagorean triples: (m^2 - n^2, 2mn, m^2 + n^2)
-- m and n are coprime and of opposite parity

compute = length $ filter ((==1) . length) $ group $ sort [perimeter * k |
    m <- [2..1000],
    n <- [1..m - 1],
    odd (m + n),
    gcd m n == 1,
    let perimeter = 2 * m * (m + n),
    k <- [1..1500000 `div` perimeter]] -- scale multiplier

euler075 :: IO String
euler075 = return $ show compute