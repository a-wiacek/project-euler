module Solutions.Euler348 where
import Utils.Numeric(isPalindrome)
import Data.List

check = group $ sort [p | a <- [2..40000], b <- [2..4000], let p = a^2 + b^3, isPalindrome p]

euler348 :: IO String
euler348 = return $ show $ sum $ map head $ filter ((==4) . length) check