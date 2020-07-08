module Solutions.Euler045 where
import Utils.List(ascendingIntersection)

tri = map (\x -> x * (x + 1) `div` 2) [286..]
pen = map (\x -> x * (3 * x - 1) `div` 2) [166..]
hex = map (\x -> x * (2 * x - 1)) [144..]

euler045 :: IO String
euler045 = return $ show $ head $ tri `ascendingIntersection` pen `ascendingIntersection` hex