module Solutions.Euler091 where

type Point = (Int, Int)
dot :: Point -> Point -> Int
dot (a, b) (c, d) = a * c + b * d
sub :: Point -> Point -> Point
sub (a, b) (c, d) = (a - c, b - d)

rightTriangle :: Point -> Point -> Point -> Bool
rightTriangle a b c =
    let x = sub a b
        y = sub b c
        z = sub c a
    in elem 0 [dot x y, dot y z, dot z x]

points = [(a, b) | a <- [0..50], b <- [0..50]]

o = (0, 0)
count = length [() |
    a <- points,
    b <- points,
    a /= b, a /= o, b /= o, rightTriangle a b o] `div` 2

euler091 :: IO String
euler091 = return $ show count