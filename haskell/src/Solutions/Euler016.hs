module Solutions.Euler016 where
import Data.Char

euler016 :: IO String
euler016 = return $ show $ sum $ map digitToInt $ show $ 2^1000