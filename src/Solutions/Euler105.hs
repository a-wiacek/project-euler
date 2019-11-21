module Solutions.Euler105 where
import Utils.Input(getInput)
import Solutions.Euler103(testSet)

read' :: String -> [Int]
read' s = read $ "[" ++ s ++ "]"

euler105 :: IO String
euler105 = show . sum . map sum . filter testSet . map read' . lines <$> getInput 105