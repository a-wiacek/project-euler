module Solutions.Euler247 where
import qualified Data.Set as Set

type Coords = (Double, Double)
type Index = (Int, Int)
data Area = Area
    { -- Left bottom corner of area.
      lbcorner :: !Coords
      -- Index of to-be-produced square.
    , areaIndex :: !Index
      -- Precomputed value, if (x, y) == lbcorner, then (x + squareShift, y + squareShift)
      -- is right upper corner of to-be-produced square.
    , squareShift :: !Double 
    } deriving Eq
instance Ord Area where compare a1 a2 = compare (squareShift a1) (squareShift a2)
data Square = Square { num :: !Int, squareIndex :: !Index } deriving (Eq, Ord)

-- Positive solution t to (x + t)(y + t) = 1
mkSquareShift :: Coords -> Double
mkSquareShift (x, y) = let s = x + y in (sqrt (s^2 - 4 * x * y + 4) - s) / 2
mkArea :: Coords -> Index -> Area
mkArea c i = Area c i (mkSquareShift c)

-- Build a square in area and split it to two smaller areas: above and to the right
splitArea :: Area -> (Area, Area)
splitArea a = (mkArea (x, y + t) (ix + 1, iy), mkArea (x + t, y) (ix, iy + 1))
    where (x, y) = lbcorner a
          (ix, iy) = areaIndex a
          t = squareShift a

type State = (Set.Set Area, Set.Set Square)

runSquares :: Int -> Int
runSquares bound =
    let loop :: Int -> State -> State
        loop n s@(aSet, sqSet)
            | n == bound = s
            | otherwise = loop (n + 1) (aSet'', sqSet')
            where (largestArea, aSet') = Set.deleteFindMax aSet
                  (newArea1, newArea2) = splitArea largestArea
                  aSet'' = Set.insert newArea1 $ Set.insert newArea2 aSet'
                  sqSet' = Set.insert (Square n (areaIndex largestArea)) sqSet
        (_, allSquares) = loop 1 (Set.singleton (mkArea (0, 1) (0, 0)), Set.empty)
    in num $ Set.findMax $ Set.filter (\s -> squareIndex s == (3, 3)) allSquares

euler247 :: IO String
euler247 = return $ show $ runSquares 1000000 -- Sufficient bound, discovered experimentally