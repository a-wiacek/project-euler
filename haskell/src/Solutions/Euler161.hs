module Solutions.Euler161 where
import Utils.Operators((<&&>))
import Control.Monad.Memo
import Data.Bits
import qualified Data.Map.Strict as Map
import Data.Maybe
import Data.WideWord.Word128

w0 = Word128 0 0
w1 = Word128 0 1
maxX = 9 :: Int
maxY = 12 :: Int
cellBound = maxX * maxY :: Int
wBound = w1 `shift` cellBound :: Word128

type Cell = Int -- number of bit in Word128, starting from 0
xplus, xminus, yplus, yminus :: Cell -> Cell
xplus = succ
xminus = pred
yplus = (+maxX)
yminus = subtract maxX

type Board = Word128 -- 0 -> free, 1 -> taken
initBoard, finalBoard :: Board
initBoard = w0
finalBoard = wBound - w1

takeFree :: Board -> Cell
takeFree = countTrailingZeros . complement

freeAt :: Board -> Cell -> Bool
freeAt board cell = not $ testBit board cell

updateBoard :: Cell -> Board -> Board
updateBoard = flip setBit

type Triomino = [Cell]
triominoesOnCell :: Cell -> [Triomino]
triominoesOnCell cell =
    -- 3x1 triominoes
    [ [xplus $ xplus cell, xplus cell, cell]
    , [yplus $ yplus cell, yplus cell, cell]
    , [yminus $ yminus cell, yminus cell, cell]
    , [yminus $ yminus cell, yminus cell, cell]
    , [xplus cell, cell, xminus cell]
    , [yplus cell, cell, yminus cell]
    -- L-triominoes, cell is in the center
    , [xplus cell, cell, yminus cell]
    , [xplus cell, cell, yplus cell]
    , [xminus cell, cell, yminus cell]
    , [xminus cell, cell, yplus cell]
    -- L-triominoes, cell is not in the center
    , [cell, xplus cell, yplus $ xplus cell]
    , [cell, xplus cell, yminus $ xplus cell]
    , [cell, xminus cell, yminus $ xminus cell]
    , [cell, xminus cell, yplus $ xminus cell]
    , [cell, yplus cell, xminus $ yplus cell]
    , [cell, yplus cell, xplus $ yplus cell]
    , [cell, yminus cell, xplus $ yminus cell]
    , [cell, yminus cell, xminus $ yminus cell] ]

tryToFit :: Board -> Triomino -> Maybe Board
tryToFit board triomino
    | all (freeAt board) triomino = Just $ foldr updateBoard board triomino
    | otherwise = Nothing

connectedCells :: Cell -> Cell -> Bool
connectedCells a' b' =
    let a = min a' b'
        b = max a' b'
    in b - a == maxX || (b - a == 1 && b `mod` maxX /= 0)
connectedTriomino :: Triomino -> Bool
connectedTriomino [a, b, c] = (==2) $ length $ filter (uncurry connectedCells) [(a, b), (b, c), (c, a)]
validateCell :: Cell -> Bool
validateCell = (<cellBound) <&&> (>=0)
validateTriomino :: Triomino -> Bool
validateTriomino = all validateCell <&&> connectedTriomino
validTriominoes :: Cell -> [Triomino]
validTriominoes = filter validateTriomino . triominoesOnCell
filledBoards :: Board -> [Board]
filledBoards board = mapMaybe (tryToFit board) $ validTriominoes $ takeFree board

initCache :: Map.Map Board Int
initCache = Map.singleton finalBoard 1

compute :: Board -> Memo Board Int Int
compute board = sum <$> forM (filledBoards board) (memo compute)

euler161 :: IO String
euler161 = return $ show $ evalMemo (compute initBoard) initCache