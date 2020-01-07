module Solutions.Euler015 where
import Utils.Numeric(binom)

euler015 :: IO String
euler015 = return $ show $ binom 40 20