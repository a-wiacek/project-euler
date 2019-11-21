module Solutions.Euler504 where
import Utils.Numeric(isSquare)

-- 1. Pick's theorem
-- 2. Given two lattice points A = (a, b) and X = (x, y),
--    there are gdc (x - a) (y - b) + 1 lattice points on it.
--    In code (+1) does not appear - we don't want to count ends twice.

f a b c d = let area2 = (a + c) * (b + d)
                boundPoints = gcd a b + gcd b c + gcd c d + gcd d a
            in isSquare $ (area2 + 2 - boundPoints) `div` 2

compute m = let l = [1..m] in length $ filter id [f a b c d | a <- l, b <- l, c <- l, d <- l]

euler504 :: IO String
euler504 = return $ show $ compute 100