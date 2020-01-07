module Solutions.Euler013 where
import Utils.Input(getInput)

euler013 :: IO String
euler013 = take 10 . show . sum . map read . lines <$> getInput 13