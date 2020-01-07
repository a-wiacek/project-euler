module Solutions.Euler056 where
import Data.Char

euler056 :: IO String
euler056 = return $ show $ maximum [sum $ map digitToInt $ show $ a^b | a <- [1..100], b <- [1..100]]