module Solutions.Euler018 where
import Utils.Input(getInput)

calcTwoRows :: [Int] -> [Int] -> [Int]
calcTwoRows row1 row2 =
    let go _ [] acc = reverse acc
        go (a:b:t1) (c:t2) acc = go (b:t1) t2 ((max a b + c):acc)
    in go row1 row2 []

calcMax :: [[Int]] -> Int
calcMax xss = case xss of
    [[a]] -> a
    h1:h2:t -> calcMax $ calcTwoRows h1 h2:t

process = calcMax . reverse . map (map read . words) . lines

euler018 :: IO String
euler018 = show . process <$> getInput 18