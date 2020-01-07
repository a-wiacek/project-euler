module Solutions.Euler055 where
import Utils.Numeric(reverseInt, isPalindrome)

lynchrel iter n =
    iter == 0 || let n' = reverseInt n + n
                 in not (isPalindrome n') && lynchrel (iter - 1) n'

euler055 :: IO String
euler055 = return $ show $ length $ filter (lynchrel 50) [1..9999]