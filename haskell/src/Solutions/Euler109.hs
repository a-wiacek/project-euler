module Solutions.Euler109 where
import Control.Monad

data Dart = Single Int | Double Int | Triple Int deriving (Eq, Ord, Show)
type Throw = [Dart]
dartVal (Single x) = x
dartVal (Double x) = 2 * x
dartVal (Triple x) = 3 * x
isDouble throw = case last throw of
    Double _ -> True
    _ -> False

darts = [Single 25, Double 25] ++ ([Single, Double, Triple] <*> [1..20])

checkoutWith n = length [() |
    i <- [1..3],
    throw <- replicateM i darts,
    i /= 3 || head throw >= head (tail throw),
    sum (map dartVal throw) == n,
    isDouble throw]

checkoutWithUpTo n = sum $ map checkoutWith [2..n]

euler109 :: IO String
euler109 = return $ show $ checkoutWithUpTo 99