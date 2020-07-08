module Solutions.Euler099 where
import Utils.Input(getInput)
import Utils.List(maxBy, splitOn)

power :: String -> Integer
power str = let [a, b] = splitOn ',' str in read a ^ read b

euler099 :: IO String
euler099 = show . fst . maxBy snd . zip [1..] . map power . words <$> getInput 99