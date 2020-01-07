module Solutions.Euler029 where
import Utils.List(uniques)

euler029 :: IO String
euler029 = return $ show $ length $ uniques [a^b | a <- [2..100], b <- [2..100]]