{-# LANGUAGE GeneralizedNewtypeDeriving #-}
module Solutions.Euler189 where
import Utils.Array(modifyArray)
import Utils.Operators((<&&>))
import Control.Monad
import Data.Array.ST
import Data.Array.Unboxed
import Data.Word
import Data.Word8

{- Strategy: Construct 4x4 triangles and match 4 of them
                ^
               / \
              / 0 \
             /     \
            ---------
           / \     / \
          / 1 \ 2 / 3 \
         /     \ /     \
  edgeL ----------------- edgeR
       / \     / \     / \
      / 4 \ 5 / 6 \ 7 / 8 \
     /     \ /     \ /     \
    -------------------------
   / \     / \     / \     / \
  / 9 \ A / B \ C / D \ E / F \
 /     \ /     \ /     \ /     \
---------------------------------
             edgeB
Rotate 180 degrees for down view
-}

-------------------------------- Types 

newtype Colour = Colour { unColour :: Word8 } deriving Eq
newtype Row = Row { unRow :: Word8 } deriving (Eq, Ord, Ix, Enum)
newtype Triangle = Triangle { unTriangle :: Word32 } deriving (Eq, Enum)

-------------------------------- Type bounds, accesors and constructors

-- Colour
allColours = [Colour 0, Colour 1, Colour 2] -- red, green, blue

-- Row
minRow = Row 0
maxRow = Row 80
allRows = [minRow..maxRow]
mkRow :: Colour -> Colour -> Colour -> Colour -> Row
mkRow col1 col2 col3 col4 = Row $ sum $ zipWith (*) [1, 3, 9, 27] $ map unColour [col1, col2, col3, col4]

unCol :: Word8 -> Row -> Colour
unCol d row = Colour $ mod (div (unRow row) d) 3
unCol1, unCol2, unCol3, unCol4 :: Row -> Colour
unCol1 = unCol 1
unCol2 = unCol 3
unCol3 = unCol 9
unCol4 = unCol 27

matchingRows :: Row -> Row -> Bool
matchingRows row1 row2 = unCol1 row1 /= unCol1 row2 && unCol2 row1 /= unCol2 row2
                      && unCol3 row1 /= unCol3 row2 && unCol4 row1 /= unCol4 row2 

-- Triangle
minTriangle = Triangle 0
maxTriangle = Triangle $ 3^16 - 1
allTriangles = [minTriangle..maxTriangle]
mkTriangle :: [Colour] -> Triangle
mkTriangle = Triangle . sum . zipWith (*) (map (3^) [0..]) . map (fromIntegral . unColour)

unCell :: Word32 -> Triangle -> Colour
unCell d triangle = Colour $ fromIntegral $ mod (div (unTriangle triangle) d) 3
unCell0, unCell1, unCell2, unCell3, unCell4, unCell5 :: Triangle -> Colour
unCell6, unCell7, unCell8, unCell9, unCellA, unCellB :: Triangle -> Colour
unCellC, unCellD, unCellE, unCellF :: Triangle -> Colour
unCell0 = unCell 1
unCell1 = unCell 3
unCell2 = unCell 9
unCell3 = unCell 27
unCell4 = unCell 81
unCell5 = unCell 243
unCell6 = unCell 729
unCell7 = unCell 2187
unCell8 = unCell 6561
unCell9 = unCell 19683
unCellA = unCell 59049
unCellB = unCell 177147
unCellC = unCell 531441
unCellD = unCell 1594323
unCellE = unCell 4782969
unCellF = unCell 14348907

edgeB, edgeL, edgeR :: Triangle -> Row
edgeB t = mkRow (unCell9 t) (unCellB t) (unCellD t) (unCellF t)
edgeL t = mkRow (unCellF t) (unCell8 t) (unCell3 t) (unCell0 t)
edgeR t = mkRow (unCell0 t) (unCell1 t) (unCell4 t) (unCell9 t)

-------------------------------- Task solving functions

cmpCells :: (Triangle -> Colour, Triangle -> Colour) -> Triangle -> Bool
cmpCells (cellA, cellB) triangle = cellA triangle /= cellB triangle

valid4Triangles :: [Triangle]
valid4Triangles =
    let edges = [ (unCell0, unCell2)
                , (unCell1, unCell2)
                , (unCell2, unCell3)
                , (unCell1, unCell5)
                , (unCell3, unCell7)
                , (unCell4, unCell5)
                , (unCell5, unCell6)
                , (unCell6, unCell7)
                , (unCell7, unCell8)
                , (unCell4, unCellA)
                , (unCell6, unCellC)
                , (unCell8, unCellE)
                , (unCell9, unCellA)
                , (unCellA, unCellB)
                , (unCellB, unCellC)
                , (unCellC, unCellD)
                , (unCellD, unCellE)
                , (unCellE, unCellF)
                ]
    in filter (foldr1 (<&&>) $ map cmpCells edges) allTriangles

{- Note: GHC runs out of memory while trying to compile this
valid4Triangles :: [Triangle]
valid4Triangles = map mkTriangle
    [[c0, c1, c2, c3, c4, c5, c6, c7, c8, c9, cA, cB, cC, cD, cE, cF] |
    c0 <- allColours,
    c1 <- allColours,
    c2 <- allColours,
    c0 /= c2,
    c1 /= c2,
    c3 <- allColours,
    c2 /= c3,
    c4 <- allColours,
    c5 <- allColours,
    c1 /= c5,
    c4 /= c5,
    c6 <- allColours,
    c5 /= c6,
    c7 <- allColours,
    c3 /= c7,
    c6 /= c7,
    c8 <- allColours,
    c7 /= c8,
    c9 <- allColours,
    cA <- allColours,
    c4 /= cA,
    c9 /= cA,
    cB <- allColours,
    cA /= cB,
    cC <- allColours,
    c6 /= cC,
    cB /= cC,
    cD <- allColours,
    cC /= cD,
    cE <- allColours,
    c8 /= cE,
    cD /= cE,
    cF <- allColours,
    cE /= cF]
-}

arrayTriangleWithEdge :: UArray Row Int
arrayTriangleWithEdge = runSTUArray $ do
    arr <- newArray (minRow, maxRow) 0
    forM_ valid4Triangles $ \triangle ->
        let row = edgeB triangle
        in modifyArray arr row succ
    return arr

matchableTriangles :: UArray Row Int
matchableTriangles = runSTUArray $ do
    arr <- newArray (minRow, maxRow) 0
    forM_ allRows $ \row1 -> forM_ allRows $ \row2 -> when (matchingRows row1 row2) $
        readArray arr row1 >>= writeArray arr row1 . (+arrayTriangleWithEdge ! row2)
    return arr

match4Triangles :: Triangle -> Int
match4Triangles triangle = product $ map (\edge -> matchableTriangles ! edge triangle) [edgeB, edgeR, edgeL]

count8Triangles :: Int
count8Triangles = sum $ map match4Triangles valid4Triangles

euler189 :: IO String
euler189 = return $ show count8Triangles