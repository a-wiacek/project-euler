module Solutions.Euler053 where
import Utils.Numeric(binom)

euler053 :: IO String
euler053 = return $ show $ length [() | x <- [1..100], y <- [0..x], binom x y > 1000000]