module Solutions.Euler017 where

first :: [String]
first = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
        "seventeen", "eighteen", "nineteen"]

dozens :: [String]
dozens = ["twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"]

itos :: Int -> String
itos n
    | n < 20 = first !! (n - 1)
    | n >= 100 = let prefix = first !! (n `div` 100 - 1) ++ "hundred" in
        if n `mod` 100 == 0 then prefix else prefix ++ "and" ++ itos (n `mod` 100)
    | otherwise = dozens !! (n `div` 10 - 2) ++ suffix where
        suffix = let d = n `mod` 10 in if d == 0 then "" else first !! (d - 1)

euler017 :: IO String
euler017 = return $ show $ length "onethousand" + sum (map (length . itos) [1..999])