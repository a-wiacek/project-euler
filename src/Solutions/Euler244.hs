{-# LANGUAGE Strict #-}
module Solutions.Euler244 where
import Data.Array
import Data.Char(ord, intToDigit)
import Data.List(delete, findIndex, findIndices)
import Data.Maybe(catMaybes, fromJust)
import qualified Data.IntMap.Strict as IntMap
import qualified Data.IntSet as IntSet
import Numeric(showIntAtBase)
import Utils.List(ascendingMinus, ascendingSum, foldDescend)
{-
-------------
| 0| 1| 2| 3|
-------------
| 4| 5| 6| 7|
-------------
| 8| 9|10|11|
-------------
|12|13|14|15|
-------------

nodes ~ positions
vertices ~ moves
Number of vertices in graph that we are investigating is 16 * binom(15, 7) = 102960
Each node has at most 4 edges.
-}

data Position = Position
    { blank :: Int
    , reds :: [Int]
    }

-- Encoding position as int: \sum_{i = 0}^15 a_i3^i, where a_i depends on colour of the field:
--  * blue -> 0
--  * red -> 1
--  * blank -> 2

blues :: Position -> [Int]
blues pos = ascendingMinus (delete (blank pos) [0..15]) (reds pos)

isBlue, isRed, isBlank :: Int -> Position -> Bool
isBlue n pos = n `elem` blues pos
isRed n pos = n `elem` reds pos
isBlank n pos = n == blank pos

posToIndex :: Position -> Int
posToIndex (Position b rs) = 2 * 3 ^ b + sum (map (3^) rs)

indexToPos :: Int -> Position
indexToPos n = Position (fromJust $ findIndex (=='2') s) (findIndices (=='1') s) where
    s' = showIntAtBase 3 intToDigit n ""
    s = reverse $ replicate (16 - length s') '0' ++ s'

move exclCond newBlank pos@(Position b rs)
    | exclCond b = Nothing
    | otherwise = Just $ Position b' $ if isBlue b' pos then rs else ascendingSum [b] (delete b' rs)
    where b' = newBlank b

-- Move red/blue tile in the direction (moves blank in the opposite direction)
moveUp, moveDown, moveLeft, moveRight :: Position -> Maybe Position
moveUp = move (>11) (+4)
moveDown = move (<4) (subtract 4)
moveLeft = move (\b -> b `mod` 4 == 3) succ
moveRight = move (\b -> b `mod` 4 == 0) pred

neighboursPos :: Position -> [Position]
neighboursPos pos = catMaybes $ map ($ pos) [moveUp, moveDown, moveLeft, moveRight]

neighboursIndexes :: Int -> [Int]
neighboursIndexes = map posToIndex . neighboursPos . indexToPos 

initPos, finalPos :: Position
initPos = Position 0 [1, 4, 5, 8, 9, 12, 13]
finalPos = Position 0 [2, 5, 7, 8, 10, 13, 15]
initIndex, finalIndex :: Int
initIndex = posToIndex initPos 
finalIndex = posToIndex finalPos

-- Assuming that second position was created from the first position by making one move,
-- find that move and return ASCII value of the move.

findMove :: Position -> Position -> Int
findMove prePos postPos = case blank postPos - blank prePos of
    4 -> ord 'U'
    -4 -> ord 'D'
    1 -> ord 'L'
    -1 -> ord 'R'

updateChecksum :: Int -> Int -> Int
updateChecksum checksum ord = (243 * checksum + ord) `mod` 100000007

positionsBFS :: IntMap.IntMap Int
positionsBFS = go (IntSet.singleton initIndex) 1 (IntMap.singleton initIndex 0) where
    go poss i degs
        | IntSet.member finalIndex poss = IntMap.insert finalIndex i degs
        | otherwise = go nposs (i + 1) ndegs
        where nposs = IntSet.difference (IntSet.fromList $ concatMap neighboursIndexes $ IntSet.toList poss)
                                        (IntMap.keysSet degs)
              ndegs = IntMap.union (IntMap.fromSet (const i) nposs) degs

maxDepth = IntMap.foldr max 0 positionsBFS -- 33

retrievedPaths :: [[Position]]
retrievedPaths = map (map indexToPos) (go (maxDepth - 1) [[finalIndex]]) where
    go depth paths
        | depth == 0 = paths
        | otherwise = go (depth - 1) [nbi:path
            | path@(h:_) <- paths
            , nbi <- filter (\i -> positionsBFS IntMap.!? i == Just (depth - 1)) (neighboursIndexes h)
            ]

getPathVal :: [Position] -> Int
getPathVal = foldl updateChecksum 0 . foldDescend findMove

euler244 :: IO String
euler244 = return $ show $ sum $ map getPathVal retrievedPaths