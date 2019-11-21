module Solutions.Euler166 where
import Utils.Operators((<&&>))

{- Idea: plug in 9 values, compute rest
abcd
efgh
ijkl
mnop
-}

isDigit = (>=0) <&&> (<10)

validSquares = [() |
    c <- [0..9],
    d <- [0..9],
    e <- [0..9],
    f <- [0..9],
    g <- [0..9],
    j <- [0..9],
    l <- [0..9],
    m <- [0..9],
    let s = d + g + j + m,
    let h = s - e - f - g,
    isDigit h,
    let p = s - d - h - l,
    isDigit p,
    o <- [0..9],
    let k = s - c - g - o,
    isDigit k,
    let i = s - j - k - l,
    isDigit i,
    let n = s - m - o - p,
    isDigit n,
    let b = s - f - j - n,
    isDigit b,
    let a = s - b - c - d,
    isDigit a,
    a + f + k + p == s,
    a + e + i + m == s]

euler166 :: IO String
euler166 = return $ show $ length validSquares