module Solutions.Euler068 where
import Control.Monad(guard)
import Data.List(permutations)

nextCycle (h:t) = t ++ [h]
cycleUntil el l = if head l == el
    then l
    else cycleUntil el $ nextCycle l

min' (a, b) (c, d) = if a < c then (a, b) else (c, d)

compute = maximum $ filter ((==16) . length . show) $ do
    [l0, l1, l2, l3, l4, l5, l6, l7, l8, l9] <- permutations [1..10]
    let w1 = (l0, [l0, l1, l3])
        w2 = (l2, [l2, l3, l5])
        w3 = (l4, [l4, l5, l7])
        w4 = (l6, [l6, l7, l9])
        w5 = (l8, [l8, l9, l1])
        s = l0 + l1 + l3
    guard $ all ((==s) . sum . snd) [w2, w3, w4, w5]
    let ws = [w1, w2, w3, w4, w5]
        wmin = foldr1 min' ws
        ws' = cycleUntil wmin ws
    return $ (read :: String -> Int) $ concatMap (concatMap show . snd) ws'

euler068 :: IO String
euler068 = return $ show compute