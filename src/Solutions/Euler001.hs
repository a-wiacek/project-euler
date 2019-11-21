module Solutions.Euler001 where
import Utils.Operators((<||>))

zMod :: Int -> Int -> Bool
zMod r x = x `mod` r == 0

euler001 :: IO String
euler001 = return $ show $ sum $ filter (zMod 3 <||> zMod 5) [1..999]