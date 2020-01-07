module Solutions.Euler020 where
import Data.Char

euler020 :: IO String
euler020 = return $ show $ sum $ map digitToInt $ show $ product [1..100]