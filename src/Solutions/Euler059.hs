module Solutions.Euler059 where
import Utils.Input(getInput)
import Control.Monad(replicateM)
import Data.Bits(xor)
import Data.Char(ord)
import Data.List(isInfixOf)

decrypt :: String -> Int
decrypt encryptedText =
    let encryptedASCII = read $ "[" ++ encryptedText ++ "]"
        word = map ord "Euler"
    in head [sum rawASCII |
        l <- replicateM 3 [97..122],
        let rawASCII = zipWith xor (cycle l) encryptedASCII,
        word `isInfixOf` rawASCII]

euler059 :: IO String
euler059 = show . decrypt <$> getInput 59