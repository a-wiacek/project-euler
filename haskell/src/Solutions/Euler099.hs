module Solutions.Euler099 where
import Utils.Input(getInput)
import Utils.List(splitOn)

power :: String -> Integer
power str = let [a, b] = splitOn ',' str in read a ^ read b
    
max' (a, b) (c, d) = if b > d then (a, b) else (c, d)

euler099 :: IO String
euler099 = show . fst . foldr1 max' . zip [1..] . map power . words <$> getInput 99