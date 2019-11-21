module Solutions.Euler172 where
import Utils.Numeric(binom, factorial)

u n = sum [ans |
    a <- [0..n `div` 3], -- digits appearing thrice
    b <- [0..n `div` 2], -- digits appearing twice
    c <- [0..n], -- digits appearing once
    let d = 9 - a - b - c, -- digits not appearing
    d >= 0,
    3 * a + 2 * b + c == n,
    -- choose digits appearing x times * choose their places * choose their alignment
    let ans = binom 9 a * binom n (3 * a) * factorial (3 * a) `div` (factorial 3 ^ a)
            * binom (9 - a) b * binom (n - 3 * a) (2 * b) * factorial (2 * b) `div` (factorial 2 ^ b)
            * binom (9 - a - b) c * factorial c]

euler172 :: IO String
euler172 = return $ show $ 9 * (u 17 + 17 * u 16 + 17 * 8 * u 15)