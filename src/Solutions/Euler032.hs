module Solutions.Euler032 where
import Utils.List(uniques)
import Utils.Numeric(digits)

addLists = zipWith (+)
compute = sum $ uniques [a * b | a <- [1..99], b <- [a + 1..3999],
    digits a `addLists` digits b `addLists` digits (a * b) == 0 : replicate 9 1]

euler032 :: IO String
euler032 = return $ show compute