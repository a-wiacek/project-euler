{-# LANGUAGE FlexibleContexts #-}
module Solutions.Euler107 where
import Utils.Input(getInput)
import Utils.Monad(ifteM)
import Control.Monad
import Control.Monad.ST
import Data.Array.ST
import Data.List
import Data.STRef

-- Kruskall algorithm
type Edge = ((Int, Int), Int)
type Graph = (Int, [Edge])

replaceHyphen :: Char -> Char
replaceHyphen c = if c == '-' then '0' else c
readLine :: String -> [Int]
readLine s = read $ "[" ++ map replaceHyphen s ++ "]"
makeIx :: Int -> [Int] -> [Edge]
makeIx a = zipWith (\i w -> ((a, i), w)) [1..]
properEdge :: Edge -> Bool
properEdge ((b, e), w) = b < e && w > 0
readGraph :: String -> Graph
readGraph s = (length s, filter properEdge . concat . zipWith makeIx [1..] . map readLine . lines $ s)

cmpEdge :: Edge -> Edge -> Ordering
cmpEdge (i1, w1) (i2, w2) = let a = compare w1 w2 in if a == EQ then compare i1 i2 else a
getSaving :: Graph -> Int
getSaving (n, edges') = runST $ do
    ans <- newSTRef 0
    parent <- newArray (1, n) Nothing :: ST s (STArray s Int (Maybe Int))
    rank <- newArray (1, n) 0 :: ST s (STArray s Int Int)
    let find x = readArray parent x
             >>= maybe (return x) (find >=> \root -> writeArray parent x (Just root) >> return root)
    let union x y = do
            xRoot <- find x
            yRoot <- find y
            xRank <- readArray rank xRoot
            yRank <- readArray rank yRoot
            case compare xRank yRank of
                LT -> writeArray parent xRoot (Just yRoot)
                EQ -> when (xRoot /= yRoot) (writeArray parent yRoot (Just xRoot) >> writeArray rank xRoot (xRank + 1))
                GT -> writeArray parent yRoot (Just xRoot)
    forM_ (sortBy cmpEdge edges') $ \((b, e), w) -> ifteM ((==) <$> find b <*> find e)
                                                          (modifySTRef' ans (+w))
                                                          (b `union` e)
    readSTRef ans

euler107 :: IO String
euler107 = show . getSaving . readGraph <$> getInput 107