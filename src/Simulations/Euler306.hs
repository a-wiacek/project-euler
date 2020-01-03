module Simulations.Euler306 where
import Control.Monad
import qualified Data.IntMap.Strict as Map
import Utils.Memo

newtype Multiset = Multiset (Map.IntMap Int) deriving (Eq, Ord, Show)

emptyMultiset :: Multiset
emptyMultiset = Multiset Map.empty

addElem :: Multiset -> Int -> Multiset
addElem (Multiset ms) n = if n > 1
    then Multiset $ Map.insertWith (+) n 1 ms
    else Multiset ms

playUsing :: Multiset -> Int -> [Multiset]
playUsing (Multiset ms) k
    | k < 4 = [Multiset ms']
    | otherwise = [addElem (addElem (Multiset ms') s) (k - 2 - s) | s <- [0 .. k `div` 2 - 1]]
    where ms' = Map.update (\x -> if x == 1 then Nothing else Just (x - 1)) k ms

split :: Multiset -> [Multiset]
split (Multiset ms) = concatMap (playUsing $ Multiset ms) (Map.keys ms)

type TreeMemo a = StrictMemo Multiset Bool a

anyM p [] = return False
anyM p (x:xs) = p x >>= \t -> if t then return True else anyM p xs

isWinningPosition :: Multiset -> TreeMemo Bool
isWinningPosition ms = if ms == emptyMultiset
    then return False
    else anyM (fmap not . memo isWinningPosition) (split ms)

findLosingPositions :: Int -> TreeMemo [Int]
findLosingPositions n = filterM (fmap not . isWinningPosition . addElem emptyMultiset) [1..n]

runSimulation306 :: IO ()
runSimulation306 = print $ startEvalMemo $ findLosingPositions 50