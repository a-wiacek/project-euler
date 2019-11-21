module Solutions.Euler012 where
import Utils.NumberTheory(numberOfDivisors)
import Data.List
import Data.Maybe

triangleNumbers :: [Int]
triangleNumbers = tr 1 where tr n = (n * (n + 1) `div` 2):tr (n + 1)

euler012 :: IO String
euler012 = return $ show $ fromJust $ find ((>500) . numberOfDivisors) triangleNumbers