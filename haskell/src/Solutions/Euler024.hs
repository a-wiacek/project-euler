module Solutions.Euler024 where
import Data.List

euler024 :: IO String
euler024 = return $ sort (permutations "0123456789") !! 999999