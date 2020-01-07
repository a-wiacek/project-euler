module Solutions.Euler098 where
import Utils.Input(getInput)
import Utils.List(combinations, uniques)
import Data.Char
import qualified Data.IntSet as IntSet
import Data.List
import qualified Data.Map.Strict as Map
import Data.Maybe

type Substitution = Map.Map Char Int
-- maximum length of word in file is 14
squares = IntSet.fromList $ takeWhile (<10^14) $ map (^2) [1..] :: IntSet.IntSet
isSquare :: Int -> Bool
isSquare n = IntSet.member n squares

cmpStr :: String -> String -> Ordering
cmpStr s1 s2
    | length s1 < length s2 = GT
    | length s1 > length s2 = LT
    | length s1 == length s2 = compare s1 s2
grpStr :: String -> String -> Bool
grpStr s1 s2 = length s1 == length s2
sieve :: [String] -> [[String]]
sieve = groupBy grpStr . sortBy cmpStr

singleSubstitute :: Substitution -> Char -> Char
singleSubstitute subst c = intToDigit $ fromJust $ Map.lookup c subst

substitute :: Substitution -> String -> String -> Maybe Int
substitute subst s1 s2 =
    let n1 = read $ map (singleSubstitute subst) s1
        n2 = read $ map (singleSubstitute subst) s2
    in if isSquare n1 && isSquare n2
        then Just $ max n1 n2
        else Nothing

isMappedToZero :: Substitution -> Char -> Bool
isMappedToZero subst c = Map.lookup c subst == Just 0
        
generateSubstitutions :: String -> String -> [Substitution]
generateSubstitutions trailingLetters allLetters = [subst |
    digitsSubset <- combinations (length allLetters) [0..9],
    digitsOrder <- permutations digitsSubset,
    let subst = Map.fromList $ zip allLetters digitsOrder,
    all (not . isMappedToZero subst) trailingLetters]

findSquare :: String -> String -> Maybe Int
findSquare s1 s2
    | sort s1 == sort s2 && length allLetters <= 10
        = maximum [substitute m s1 s2 | m <- generateSubstitutions trailingLetters allLetters]
    | otherwise = Nothing
    where trailingLetters = uniques $ map head [s1, s2] -- can't be substituted to 0
          allLetters = uniques s2

processSingle :: [String] -> [Int]
processSingle l = catMaybes [findSquare s1 s2 | s1 <- l, s2 <- l, s1 < s2]

process :: [[String]] -> Int
process (h:t) =
    let l = processSingle h
    in if null l
        then process t
        else maximum l

parseFile :: String -> [String]
parseFile s = read $ "[" ++ s ++ "]"

euler098 :: IO String
euler098 = show . process . sieve . parseFile <$> getInput 98