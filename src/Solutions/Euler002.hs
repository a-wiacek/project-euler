module Solutions.Euler002 where
import Utils.Numeric(fibonacci)

euler002 :: IO String
euler002 = return $ show $ sum $ filter even $ takeWhile (<4000000) $ fibonacci 1 2