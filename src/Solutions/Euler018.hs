module Solutions.Euler018 where
import Utils.Input(getInput)

calcTwoRows :: [Int] -> [Int] -> [Int]
calcTwoRows row1 row2 =
    let __c _ [] acc = reverse acc
        __c (a:b:t1) (c:t2) acc = __c (b:t1) t2 ((max a b + c):acc)
    in __c row1 row2 []

calcMax :: [[Int]] -> Int
calcMax xss = case xss of
    [[a]] -> a
    h1:h2:t -> calcMax $ calcTwoRows h1 h2:t

process = calcMax . reverse . map (map read . words) . lines

euler018 :: IO String
euler018 = show . process <$> getInput 18