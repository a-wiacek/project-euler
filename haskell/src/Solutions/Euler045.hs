module Solutions.Euler045 where
import qualified Data.Set as Set

setGen f = Set.fromAscList $ map f [1..999999]
tri = setGen (\x -> x * (x + 1) `div` 2) 
pen = setGen (\x -> x * (3 * x - 1) `div` 2) 
hex = setGen (\x -> x * (2 * x - 1))

euler045 :: IO String
euler045 = return $ show . last . Set.toList $ tri `Set.intersection` pen `Set.intersection` hex