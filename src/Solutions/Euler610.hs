module Solutions.Euler610 where
import Data.List(groupBy, sortOn)
import Solutions.Euler089(minimumForm)
import Text.Printf
import Utils.Operators((<:>))

{-
Let X be a random variable which gives value of rolled number.
We have EX = 0,14 * (EX + 1000) + 0,86 * E[X | M was not rolled first]
-> EX = 140 / 0,86 + E[X | M was not rolled first]
-}

type Numeral = (Double, String)
numerals = map (fromIntegral <:> (++"#") . minimumForm) [0..999] :: [Numeral]

data Tree = Node Double [Tree]

buildSuffixTree :: [Numeral] -> Tree
buildSuffixTree l =
    let [(d, "#")] : l' = groupBy (\x y -> head (snd x) == head (snd y)) l
    in Node d $ map (buildSuffixTree . map (fmap tail)) l'

expectedValue :: Tree -> Double
expectedValue (Node d ds) =
    let w = fromIntegral $ 7 * length ds
    in (d + 7 * sum (map expectedValue ds)) / (w + 1)

theExpectedValue = expectedValue $ buildSuffixTree $ sortOn snd $ numerals

euler610 :: IO String
euler610 = return $ printf "%.8f" (140 / 0.86 + theExpectedValue)