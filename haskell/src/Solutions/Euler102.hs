module Solutions.Euler102 where
import Utils.Input(getInput)

type Point = (Int, Int)
type Triangle = (Point, Point, Point)
area :: Triangle -> Int
area ((xa, ya), (xb, yb), (xc, yc)) =
    abs $ xa * (yb - yc) + xb * (yc - ya) + xc * (ya - yb)

o = (0, 0) :: Point
verify :: Triangle -> Bool
verify (a, b, c) = area (a, b, c) == area (a, b, o) + area (a, o, c) + area (o, b, c)

read' :: String -> Triangle
read' x = let [q, w, e, r, t, y] = read $ "[" ++ x ++ "]" in ((q, w), (e, r), (t, y))

euler102 :: IO String
euler102 = show . length . filter verify . map read' . words <$> getInput 102