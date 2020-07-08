{-# LANGUAGE FlexibleContexts, LambdaCase #-}
module Solutions.Euler095 where
import Utils.Array(modifyArray)
import Utils.List(maxBy, reduceList)
import Solutions.Euler078(pentagonal)
import Control.Monad
import Control.Monad.ST
import Data.Array.Unboxed
import Data.Array.ST
import Data.List
import Data.STRef

-- https://en.wikipedia.org/wiki/Divisor_function#Other_properties_and_identities
-- https://oeis.org/A000203

bound = 10^6
pentaIndexSigns = (-1):(-1):1:1:pentaIndexSigns

divisorsSum = runSTUArray $ do
    divSum <- newArray (0, bound) 0 :: ST s (STUArray s Int Int)
    pentagonalTracker <- newSTRef $ zip pentagonal pentaIndexSigns
    forM_ [1..bound] $ \x ->
        forM (takeWhile ((<=x) . snd) pentagonal)
             (\(mul, y) -> (*mul) <$> readArray divSum (x - y))
        >>= writeArray divSum x . sum >>
        readSTRef pentagonalTracker >>= \(((_, h), sign):t) -> when (h == x) $
            writeSTRef pentagonalTracker t >> modifyArray divSum x (subtract (sign * x))
    -- exclude largest divisor of n: n
    forM_ [1..bound] $ \x -> readArray divSum x >>= writeArray divSum x . subtract x
    return divSum
           
data VertexState = Unknown | Traversed | Uncycled | Cycle Int deriving Eq

--  Go forward until you find vertex which is not Unknown. Mark traversed vertices.
--  * If vertex is Uncycled or Cycled, all traversed verices are Uncycled.
--  * If it's Traversed, we have cycle. Traverse it twice:
--    - to find minimum m,
--    - to mark vertices as Cycled m.
--  Mark tail as Uncycled.
process chain n = do
    cycleBegin <- newSTRef (-1)
    curr <- newSTRef n
    let destroy = readSTRef curr >>= \v -> when (v <= bound) $
            readArray chain v >>= \s -> when (s == Traversed) $
                writeArray chain v Uncycled >> writeSTRef curr (divisorsSum ! v) >> destroy
    let runChain = readSTRef curr >>= \v -> if v <= bound
            then readArray chain v >>= \case
                    Unknown -> writeArray chain v Traversed >> writeSTRef curr (divisorsSum ! v) >> runChain
                    Traversed -> writeSTRef cycleBegin v
                    _ -> writeSTRef curr n >> destroy
            else writeSTRef curr n >> destroy
    runChain
    cb <- readSTRef cycleBegin
    when (cb > -1) $ do
        writeSTRef curr cb
        minV <- newSTRef cb
        let runMin = do
                nextv <- (divisorsSum !) <$> readSTRef curr
                writeSTRef curr nextv
                when (nextv /= cb) $ modifySTRef' minV (min nextv) >> runMin
        runMin
        cycleMin <- readSTRef minV
        let runMarkCycled = do
                v <- readSTRef curr
                s <- readArray chain v
                let nextv = divisorsSum ! v
                when (s == Traversed) $ writeArray chain v (Cycle cycleMin) >> writeSTRef curr nextv >> runMarkCycled
        runMarkCycled
        writeSTRef curr n
        destroy

divisorsCycles = runSTArray $ do
    chain <- newArray (1, bound) Unknown :: ST s (STArray s Int VertexState)
    writeArray chain 1 Uncycled
    forM_ [2..bound] $ process chain
    return chain

justCycle :: [VertexState] -> [Int]
justCycle l = [n | Cycle n <- l]

euler095 :: IO String
euler095 = return $ show $ fst $ maxBy snd $ map reduceList
         $ group $ sort $ justCycle $ elems divisorsCycles