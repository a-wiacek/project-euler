module Solutions.Euler039 where

countSolutions n = (n, sols) where
    sols = length [() | a <- [1..n - 1],
                        b <- [a + 1..n - a],
                        let c = n - a - b,
                        a * a + b * b == c * c]

max' (a, b) (c, d) = if b > d then (a, b) else (c, d)

euler039 :: IO String
euler039 = return $ show $ fst $ foldr1 max' $ map countSolutions [1..1000]