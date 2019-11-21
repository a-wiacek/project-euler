module Solutions.Euler040 where
import Data.Char

positions = [0, 9, 99, 999, 9999, 99999, 999999]

compute = let word = concatMap show [1..]
          in product $ map (digitToInt . (word !!)) positions

euler040 :: IO String
euler040 = return $ show compute