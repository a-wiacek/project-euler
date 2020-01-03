module Solutions.Euler104 where
import Utils.Numeric(fibonacci, isPandigital)
import Data.List
import Data.Maybe

check :: Integer -> Bool
check n = all isPandigital [n1, n2]
    where n1 = read $ take 9 $ show n :: Integer
          n2 = read $ take 9 $ reverse $ show n :: Integer

euler104 :: IO String
euler104 = return $ show $ fromJust $ findIndex check $ fibonacci 0 1