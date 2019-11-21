module Solutions.Euler033 where

prod' (a, b) (c, d) = (a * c, b * d)
ans (num, den) = den `div` gcd den num

compute = ans $ foldr1 prod' [(a, b) |
    a <- [11..99],
    a `mod` 10 /= 0,
    b <- [a + 1..99],
    b `mod` 10 /= 0,
    a `mod` 10 == b `div` 10,
    a * (b `mod` 10) == b * (a `div` 10)]

euler033 :: IO String
euler033 = return $ show compute