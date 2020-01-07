module Solutions.Euler089 where
import Utils.Input(getInput)

minimumForm :: Int -> String
minimumForm n
    | n >= 1000 = "M" ++ minimumForm (n - 1000)
    | n < 1000 && n >= 900 = "CM" ++ minimumForm (n - 900)
    | n < 900 && n >= 500 = "D" ++ minimumForm (n - 500)
    | n < 500 && n >= 400 = "CD" ++ minimumForm (n - 400)
    | n < 400 && n >= 100 = "C" ++ minimumForm (n - 100)
    | n < 100 && n >= 90 = "XC" ++ minimumForm (n - 90)
    | n < 90 && n >= 50 = "L" ++ minimumForm (n - 50)
    | n < 50 && n >= 40 = "XL" ++ minimumForm (n - 40)
    | n < 40 && n >= 10 = "X" ++ minimumForm (n - 10)
    | n < 10 = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"] !! n

readRoman :: String -> Int
readRoman s = case s of
    'M':t -> 1000 + readRoman t
    'C':'M':t -> 900 + readRoman t
    'D':t -> 500 + readRoman t
    'C':'D':t -> 400 + readRoman t
    'C':t -> 100 + readRoman t
    'X':'C':t -> 90 + readRoman t
    'L':t -> 50 + readRoman t
    'X':'L':t -> 40 + readRoman t
    'X':t -> 10 + readRoman t
    'I':'X':t -> 9 + readRoman t
    'V':t -> 5 + readRoman t
    'I':'V':t -> 4 + readRoman t
    'I':t -> 1 + readRoman t
    "" -> 0

shorten :: String -> String
shorten = minimumForm . readRoman

diffLength :: String -> Int
diffLength s = length s - length (shorten s)

euler089 :: IO String
euler089 = show . sum . map diffLength . lines <$> getInput 89