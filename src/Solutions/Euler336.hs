module Solutions.Euler336 where
import Data.List(delete)

-- All arrangements in lexigographical order
permutations :: Int -> [String]
permutations n = go (take n ['A'..]) where
    go [] = [[]]
    go l = [a:b | a <- l, b <- go (delete a l)]

splitOn :: Eq a => a -> [a] -> ([a], [a])
splitOn e [] = ([], [])
splitOn e (h:t) = if e == h then ([], h:t) else (h:p, q) where (p, q) = splitOn e t

countRotations :: String -> Int
countRotations = go 0 'A' where
    go k i [] = k
    go k i (h:t) = if h == i
        then go k (succ i) t
        else case splitOn i (h:t) of
            (l, [i]) -> go (k + 1) (succ i) (reverse l)
            (l1, l2) -> go (k + 2) (succ i) (tail $ reverse $ l1 ++ reverse l2)

findArr :: Int -> Int -> String
findArr whichArr carriages =
    (filter (\c -> countRotations c == 2 * carriages - 3) $ permutations carriages) !! (whichArr - 1)

euler336 :: IO String
euler336 = return $ findArr 2011 11