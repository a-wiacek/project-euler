module Solutions.Euler065 where
import Data.Ratio
import Data.Char

e :: [Integer]
e = 2:concat (iterate (\[a, b, c] -> [a, b + 2, c]) [1, 2, 1])

approxs :: [Integer] -> [Rational]
approxs (h:t) = let h' = h % 1 in h' : map ((+h') . (1/)) (approxs t)

euler065 :: IO String
euler065 = return $ show $ sum $ map digitToInt $ show $ numerator $ approxs e !! 99