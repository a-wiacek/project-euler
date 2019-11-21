module Solutions.Euler025 where
import Utils.Numeric(fibonacci)

euler025 :: IO String
euler025 = return $ show $ length $ takeWhile ((1000>) . length . show) $ fibonacci 0 1