module Solutions.Euler168 where
import Utils.List(uniques)

compute = flip mod 100000 $ sum $ uniques [n |
    b <- [0..9],
    k <- [2..100],
    l <- [1..10],
    let anum = b * (10^(k - 1) - l),
    let adem = 10 * l - 1,
    anum `mod` adem == 0,
    let n = 10 * anum `div` adem + b,
    length (show n) == k]

euler168 :: IO String
euler168 = return $ show compute