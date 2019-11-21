{-# LANGUAGE Strict #-}
module Solutions.Euler246 where
import Utils.Operators((<&&>))

{-
Look at associated GeoGebra file.
You can move point on the red circle and see that all interesting points are inside that circle.

https://math.stackexchange.com/questions/1932935/finding-angle-between-two-tangents-to-the-ellipse

Ellipse equation: ((x - 3000) / 7500)^2 + ((y - 1500) / (2500 * sqrt 5))^2 = 1
Equivalently: 5 * (x - 3000)^2 + 9 * (y - 1500)^2 = 5 * 7500^2
Parameterization: (7500 * cos t + 3000, 2500 * sqrt 5 * sin t + 1500)
Suppose that we're investigating point (px, py) outside of ellipse to find tangents.
Then we have: px * cos t / 7500 + py * sin t / (2500 * sqrt 5) = 1
-}

data Circle = Circle { center :: (Int, Int), radius :: Int }
f = fromIntegral :: Int -> Double

dist (a, b) (x, y) = sqrt $ (a - x)^2 + (b - y)^2
minus (a, b) (x, y) = (a - x, b - y)
dot (a, b) (x, y) = a * x + b * y

pointM = (-2000, 1500)
pointG = (8000, 1500)
pointE = (3000, 1500)
circleC = Circle pointM 15000
circleBound = Circle pointE 19000

latticePoints :: Circle -> [(Int, Int)]
latticePoints c = [(x, y) | x <- [cx - r..cx + r],
                            let t = floor $ sqrt $ f $ r^2 - (cx - x)^2,
                            y <- [cy - t..cy + t]] where
    (cx, cy) = center c
    r = radius c

ellipse :: Double -> (Double, Double)
ellipse t = (7500 * cos t + 3000, 2500 * sqrt 5 * sin t + 1500)
outsideEllipse :: (Int, Int) -> Bool
outsideEllipse (x, y) = 5 * (x - 3000)^2 + 9 * (y - 1500)^2 > 5 * 7500^2

-- (7500 * cos t + 3000 - x, 2500 * sqrt 5 * sin t + 1500 - y) x (-7500 * sin t, 2500 * sqrt 5 * cos t)
-- solve (7500 * cos t + 3000 - x) * 2500 * sqrt 5 * cos t + (2500 * sqrt 5 * sin t + 1500 - y) * 7500 * sin t = 0 for t
-- (in WolframAlpha)
properAngle :: (Int, Int) -> Bool
properAngle (x, y) = cosphi < sqrt 0.5 where
    u = sqrt $ f $ 5 * x^2 + 9 * y^2 - 30000 * x - 27000 * y - 216000000
    s = f $ 3 * y - 4500
    d = sqrt 5 * f (x + 4500)
    z1 = ellipse $ if d == 0 then pi else - 2 * atan ((u - s) / d)
    z2 = ellipse $ if d == 0 then 2 * atan (2500 * sqrt 5 / (f y - 1500)) else 2 * atan ((u + s) / d)
    p = (f x, f y)
    cosphi = (dot (minus z1 p) (minus z2 p)) / (dist z1 p * dist z2 p)

properLatticePoints = filter (outsideEllipse <&&> properAngle) (latticePoints circleBound)

euler246 :: IO String
euler246 = return $ show $ length properLatticePoints