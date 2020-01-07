module Solutions.Euler225 where
import qualified Data.Set as Set

type T = (Int, Int, Int)
step :: T -> Int -> T
step (a, b, c) m = (b, c, (a + b + c) `mod` m)

test :: Int -> Bool
test n =
    let test' :: Set.Set T -> T -> Bool
        test' set t@(a, b, c)
            | c == 0 = False
            | t `Set.member` set = True
            | otherwise = test' (t `Set.insert` set) (step t n)
    in test' Set.empty (1, 1, 1)

euler225 :: IO String
euler225 = return $ show $ filter test [27, 29..] !! 123