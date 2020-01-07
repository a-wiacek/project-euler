module Solutions.Euler165 where
import Utils.List(uniques)
import Data.Maybe

type Point = (Double, Double)
type Line = (Point, Point)

epsilon = 10e-12
createSegments [] = []
createSegments (a:b:c:d:t) = ((a, b), (c, d)) : createSegments t

shubs :: Integer -> [Integer]
shubs x = x : shubs (x * x `mod` 50515093)
segments = createSegments $ map (fromInteger . (`mod` 500)) $ take 20000 $ tail $ shubs 290797

in01 x = 0 < x && x < 1
pointInLine :: Line -> Point -> Bool
pointInLine ((xb, yb), (xe, ye)) (xm, ym) = case (xb == xe, yb == ye) of
    (True, True) -> False
    (False, True) -> in01 tx && ym == ye
    (True, False) -> xm == xe && in01 ty
    (False, False) -> abs (tx - ty) < epsilon && in01 tx
    where tx = (xm - xe) / (xb - xe)
          ty = (ym - ye) / (yb - ye)
zeroOnLine :: Line -> Bool
zeroOnLine ((xb, yb), (xe, ye)) = yb * xe == ye * xb

-- a, b, c such that ax + by + c = 0
lineThroughPoints :: Line -> (Double, Double, Double)
lineThroughPoints ((xb, yb), (xe, ye)) =
    let a = ye - yb
        b = xb - xe
        c = yb * xe - xb * ye
    in (a, b, c)

intersection :: Line -> Line -> Maybe Point
intersection l1 l2
    | w == 0 = Nothing
    | pointInLine l1 p && pointInLine l2 p = Just p
    | otherwise = Nothing
    where (a1, b1, c1) = lineThroughPoints l1
          (a2, b2, c2) = lineThroughPoints l2
          w = a1 * b2 - a2 * b1
          wx = b1 * c2 - b2 * c1
          wy = c1 * a2 - c2 * a1
          p = (wx / w, wy / w)

euler165 :: IO String
euler165 = return $ show $ length $ uniques $ catMaybes $ intersection <$> segments <*> segments