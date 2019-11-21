module Solutions.Euler043 where
import Data.List

compute = sum [read n |
    n@[d0, d1, d2, d3, d4, d5, d6, d7, d8, d9] <- permutations "0123456789",
    d0 /= '0',
    read [d1, d2, d3] `mod` 2 == 0,
    read [d2, d3, d4] `mod` 3 == 0,
    read [d3, d4, d5] `mod` 5 == 0,
    read [d4, d5, d6] `mod` 7 == 0,
    read [d5, d6, d7] `mod` 11 == 0,
    read [d6, d7, d8] `mod` 13 == 0,
    read [d7, d8, d9] `mod` 17 == 0]

euler043 :: IO String
euler043 = return $ show compute