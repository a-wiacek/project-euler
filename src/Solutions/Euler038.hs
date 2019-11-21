module Solutions.Euler038 where
import Utils.Numeric(isPandigital)
import Control.Applicative((<|>))
import Data.Maybe

getPandigital' n i =
    let k = read . concat $ show . (*n) <$> [1..i]
    in if isPandigital k then Just k else Nothing

getPandigital n = foldl1 (<|>) $ map (getPandigital' n) [2..5]

euler038 :: IO String
euler038 = return $ show $ maximum $ mapMaybe getPandigital [1..9999]