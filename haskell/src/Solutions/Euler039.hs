module Solutions.Euler039 where
import Utils.List(maxBy)

countSolutions n = (n, sols) where
    sols = length [() | a <- [1..n - 1],
                        b <- [a + 1..n - a],
                        let c = n - a - b,
                        a * a + b * b == c * c]

euler039 :: IO String
euler039 = return $ show $ fst $ maxBy snd $ map countSolutions [1..1000]