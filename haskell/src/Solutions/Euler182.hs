module Solutions.Euler182 where

unconcealed p q =
    let phi = (p - 1) * (q - 1) 
    in sum [e | e <- [2..phi - 1],
                gcd e phi == 1,
                gcd (e - 1) (p - 1) == 2,
                gcd (e - 1) (q - 1) == 2]

euler182 :: IO String
euler182 = return $ show $ unconcealed 1009 3643