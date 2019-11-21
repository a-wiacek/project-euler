module Solutions.Euler353 where
import Control.Monad
import Control.Monad.ST
import Data.Array
import Data.Array.ST
import Data.List
import Text.Printf
import Utils.Array

type Coords = (Double, Double, Double)

triples :: Int -> [Coords]
triples r = [ (fromIntegral a, fromIntegral b, fromIntegral c)
            | a <- [0..r], b <- [0..r]
            , let c2 = r * r - a * a - b * b
            , c2 >= 0
            , let c = floor $ sqrt $ fromIntegral c2
            , c * c == c2 ]

-- https://stackoverflow.com/questions/52210911/great-circle-distance-between-two-p-x-y-z-points-on-a-unit-sphere
risk :: Coords -> Coords -> Double
risk (a, b, c) (x, y, z) = (2 * phi / pi)^2
    where rawDist = sqrt $ (a - x)^2 + (b - y)^2 + (c - z)^2
          phi = asin $ rawDist / (2 * (sqrt $ a^2 + b^2 + c^2))
reflectRisk :: Coords -> Double
reflectRisk (a, b, c) = risk (a, b, c) (a, b, -c)

infty = 10**10 :: Double

-- Assumption: Road is symmetric
dijkstra :: [Coords] -> Double
dijkstra coords = runST $ do
    dist <- newArray (1, l) infty :: ST s (STUArray s Int Double)
    visited <- newArray (1, l) False :: ST s (STUArray s Int Bool) 
    writeArray dist 1 0
    let findMinIndex = forM [1..l] (\x -> readArray visited x >>= \p -> if p
                                    then return (x, infty + 1)
                                    else readArray dist x >>= \v -> return (x, v)) >>=
                       return . fst . foldr (\x@(_, dMin) y@(_, d) -> if d < dMin then y else x) (0, infty)
    let loop = findMinIndex >>= \i -> when (i > 0) $ do
                  writeArray visited i True
                  thisDist <- readArray dist i
                  forM_ [1..l] $ \j -> when (i /= j) $
                      modifyArray dist j $ min $ thisDist + risk (verticesArr ! i) (verticesArr ! j)
                  loop
    loop
    fmap (foldr min infty) $ forM [1..l] $ \i -> do
        v <- readArray dist i
        return $ 2 * v + reflectRisk (verticesArr ! i)
    where vertices = zip [1..] coords
          l = length coords
          verticesArr = array (1, l) vertices

funM :: Int -> Double
funM = dijkstra . triples

euler353 :: IO String
euler353 = return $ printf "%.10f" $ sum $ map (funM . (\x -> 2^x - 1)) [1..15]