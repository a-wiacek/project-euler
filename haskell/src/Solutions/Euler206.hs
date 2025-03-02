module Solutions.Euler206 where
import Utils.List(everyOther)

isOK = (=="123456789") . everyOther True . show . (^2)

loop n | isOK (10 * n + 3) = 100 * n + 30
       | isOK (10 * n + 7) = 100 * n + 70
       | n == 13890266 = undefined
       | otherwise = loop (n + 1)

euler206 :: IO String
euler206 = return $ show $ loop 10101010