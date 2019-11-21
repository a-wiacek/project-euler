module Solutions.Euler034 where
import Utils.Numeric(factorial)
import Data.Char

curious n = n == sum (map (factorial . digitToInt) (show n))

euler034 :: IO String
euler034 = return $ show $ sum $ filter curious [3..100000]