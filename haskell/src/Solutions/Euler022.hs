module Solutions.Euler022 where
import Utils.Input(getInput)
import Utils.List(splitOn)
import Data.List(sort)
import Data.Char(ord)

wordValue :: String -> Int
wordValue = sum . map (\c -> ord c - ord 'A' + 1)

pairValue :: Int -> String -> Int
pairValue x w = wordValue w * x

euler022 :: IO String
euler022 = show . sum . zipWith pairValue [1..] . sort . map (tail . init) . splitOn ',' <$> getInput 22