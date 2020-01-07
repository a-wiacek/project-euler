module Solutions.Euler142 where
import Utils.Numeric(isSquare)

-- x + y = a^2
-- x - y = b^2
-- x + z = c^2

bound = 1000 :: Int
compute :: Int
compute = head [x + y + z |
    a <- [1..bound],
    b <- [1..bound],
    even (a - b),
    let x = (a^2 + b^2) `div` 2,
    let y = a^2 - x,
    c <- [1..bound],
    let z = c^2 - x,
    all (>0) [x - y, y - z, z],
    all isSquare [x + z, x - z, y + z, y - z]] -- x + y and x - y are for free

euler142 :: IO String
euler142 = return $ show compute