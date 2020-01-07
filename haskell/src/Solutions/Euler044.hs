module Solutions.Euler044 where
import qualified Data.Set as Set

bound = 2500
pentagonalList = map (\x -> x * (3 * x - 1) `div` 2) [1..bound]
pentagonalSet = Set.fromAscList pentagonalList

compute = minimum [p - q |
    p <- pentagonalList,
    q <- pentagonalList,
    p > q,
    Set.member (p - q) pentagonalSet,
    Set.member (p + q) pentagonalSet]

euler044 :: IO String
euler044 = return $ show compute