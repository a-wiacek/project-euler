module Solutions.Euler070 where
import Utils.NumberTheory(totientArrayUpTo)
import Utils.Numeric(digits)
import Utils.Operators((<:>))
import Data.Array.Unboxed((!))

bound = 10000000 :: Int
totient = totientArrayUpTo bound
sameDigits :: Int -> Int -> Bool
sameDigits a b = digits (toInteger a) == digits (toInteger b)
isOk :: Int -> Bool
isOk n = sameDigits n (totient ! n)
ddiv :: Int -> Int -> Double
ddiv a b = (fromIntegral a) / (fromIntegral b)
min' (a, b) (c, d) = if b < d then (a, b) else (c, d)

euler070 :: IO String
euler070 = return $ show $ fst $ foldr (min' . (id <:> (\x -> ddiv x $ totient ! x))) (bound, 2.5) (filter isOk [2..bound])