module Solutions.Euler004 where
import Utils.Numeric(isPalindrome)

euler004 :: IO String
euler004 = return $ show $ maximum $ filter isPalindrome [x * y | x <- [100..999], y <- [x..999]]