module Solutions.Euler104 where
import Utils.Numeric(fibonacci, isPandigital)
import Data.List
import Data.Maybe

check :: Integer -> Bool
check n =
    let n1 = read $ take 9 $ show n :: Integer
        n2 = read $ take 9 $ reverse $ show n :: Integer
    in isPandigital n1 && isPandigital n2

euler104 :: IO String
euler104 = return $ show $ fromJust $ findIndex check $ fibonacci 0 1