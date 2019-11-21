module Solutions.Euler277 where
import Data.Maybe
import Data.Ratio
import Data.List

data Poly = Poly Rational Rational -- Poly a b === ax + b
infixl 7 ***
(***) :: Rational -> Poly -> Poly
n *** (Poly a b) = Poly (n * a) (n * b)
infixl 6 +++
(+++) :: Rational -> Poly -> Poly
n +++ (Poly a b) = Poly a (n + b)
eval :: Poly -> Rational -> Rational
eval (Poly a b) r = a * r + b

decipher :: String -> Poly
decipher "" = Poly 1 0
decipher ('D':t) = 3 *** decipher t
decipher ('U':t) = (1 % 4) *** ((-2) +++ 3 *** decipher t)
decipher ('d':t) = (1 % 2) *** (1 +++ 3 *** decipher t)

msg = "UDDDUdddDDUDDddDdDddDDUDDdUUDd"
p = decipher msg

leastX = fromJust $ find (\x -> denominator (eval p x) == 1) [1..]
leastA = eval p leastX
leastA' = numerator $ fromJust $ find (>10^15) [leastA, leastA + 3 ^ length msg..]

euler277 :: IO String
euler277 = return $ show leastA'

