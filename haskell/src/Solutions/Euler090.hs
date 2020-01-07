module Solutions.Euler090 where
import Utils.List(combinations)
import Data.List

squares = map (^2) [1..9]
rev c = if c `elem` [6, 9] then [6, 9] else [c]

check :: [Int] -> [Int] -> Bool
check dice1 dice2 = all (`elem` l) squares where
    l = concat [[10 * n1 + n2, 10 * n2 + n1] | n1 <- concatMap rev dice1, n2 <- concatMap rev dice2]

count = length [() | 
    dice1 <- combinations 6 [0..9],
    dice2 <- combinations 6 [0..9],
    check dice1 dice2] `div` 2

euler090 :: IO String
euler090 = return $ show count