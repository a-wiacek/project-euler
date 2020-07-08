module Solutions.Euler103 where

twoSubsets' :: [a] -> [([a], [a])]
twoSubsets' [] = [([], [])]
twoSubsets' (h:t) =
    let l = twoSubsets' t
    in l ++ [(h:l1, l2) | (l1, l2) <- l] ++ [(l1, h:l2) | (l1, l2) <- l] 
twoSubsets :: [a] -> [([a], [a])]
twoSubsets = filter (\(x, y) -> not (null x) && not (null y)) . twoSubsets'

p :: [Int] -> [Int] -> Bool
p l1 l2 = case compare (length l1) (length l2) of
    LT -> sum l1 < sum l2
    EQ -> sum l1 /= sum l2
    GT -> sum l1 > sum l2

testSet :: [Int] -> Bool
testSet l = and [p b c | (b, c) <- twoSubsets l]

bound = 50
min' (a, b) (c, d) = if a < c then (a, b) else (c, d)

compute = snd $ foldr1 min' [(sum l, l) |
    a <- [12..bound],
    b <- [a + 1..bound],
    c <- [b + 1..bound],
    d <- [c + 1..bound],
    e <- [d + 1..bound],
    f <- [e + 1..bound],
    g <- [f + 1..bound],
    let l = [a, b, c, d, e, f, g],
    testSet l]

euler103 :: IO String
euler103 = return $ concatMap show compute