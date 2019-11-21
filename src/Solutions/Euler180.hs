module Solutions.Euler180 where
import Data.Ratio
import Utils.List(uniques)

-- f_n(x, y, z) = (x^n + y^n - z^n)(x + y + z)
-- By Fermat's Last Theorem, solutions exist only if n \in \{-2, -1, 1, 2}.

bound = 35 :: Integer
ans :: Rational
ans = sum $ uniques
    [ x + y + z
    | xb <- [2..bound],
      xa <- [1..xb - 1],
      gcd xa xb == 1,
      let x = xa % xb,
      yb <- [2..bound],
      ya <- [1..yb - 1],
      gcd ya yb == 1,
      let y = ya % yb,
      x <= y,
      zb <- [2..bound],
      za <- [1..zb - 1],
      gcd za zb == 1,
      let z = za % zb,
      x^2 + y^2 == z^2 || x + y == z || 1 / x + 1 / y == 1 / z || 1 / x^2 + 1 / y^2 == 1 / z^2]

euler180 :: IO String
euler180 = return $ show $ numerator ans + denominator ans