module Solutions.Euler079 where
import Utils.Array(modifyArray)
import Utils.Input(getInput)
import Control.Monad
import Data.Array
import Data.Array.ST
import Data.Char
import Data.List
import qualified Data.Set as Set

{-
Luckily, we can notice that:
* digits 4 and 5 do not exist
* there are no cyclic dependencies (there do not exist digits a and b
  such that a needs to be BOTH before and after b)
* we can just sort digits by their before sets
-}

compare' :: (a, Set.Set b) -> (a, Set.Set b) -> Ordering
compare' (_, s1) (_, s2) = compare (Set.size s1) (Set.size s2)

process :: [[Int]] -> String
process logins = map (intToDigit . fst) $ sortBy compare' $ assocs $ runSTArray $ do
    before <- newArray (0, 9) (Set.empty :: Set.Set Int)
    forM_ logins $ \[d1, d2, d3] -> do
        modifyArray before d3 $ Set.insert d1
        modifyArray before d3 $ Set.insert d2
        modifyArray before d2 $ Set.insert d1
    return before

euler079 :: IO String
euler079 = drop 2 . process . map (map digitToInt) . words <$> getInput 79