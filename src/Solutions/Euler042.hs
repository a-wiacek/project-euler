module Solutions.Euler042 where
import Utils.Input(getInput)
import Utils.List(splitOn)
import Solutions.Euler012(triangleNumbers)
import Data.Char

tr = take 20 triangleNumbers
triangleWord w = sum (map (\c -> ord c - ord 'A' + 1) w) `elem` tr

euler042 :: IO String
euler042 = show . length . filter triangleWord . map (tail . init) . splitOn ',' <$> getInput 42