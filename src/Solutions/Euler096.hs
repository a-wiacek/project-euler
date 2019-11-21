module Solutions.Euler096 where
import Utils.Input(getInput)
import Utils.Numeric(digits)
import Utils.Operators((<&&>))
import Data.Array.Unboxed
import Data.Char

type Sudoku = UArray (Int, Int) Int
show' :: Sudoku -> String
show' sudoku =
    let split :: [Int] -> [String]
        split [] = []
        split l = map intToDigit (take 9 l):split (drop 9 l)
    in unlines $ split $ elems sudoku
sudokuPositions = [(a, b) | a <- [0..8], b <- [0..8]]

updateSudoku :: (Int, Int) -> Int -> Sudoku -> Sudoku
updateSudoku pos val sudoku = sudoku // [(pos, val)]
    
readSudoku :: [String] -> Sudoku
readSudoku lines = array ((0, 0), (8, 8)) $ zip sudokuPositions $ map digitToInt $ concat lines

splitSudokus :: [String] -> [Sudoku]
splitSudokus [] = []
splitSudokus l = readSudoku (take 9 $ drop 1 l) : splitSudokus (drop 10 l)

smallSquare :: (Int, Int) -> [(Int, Int)]
smallSquare (x, y) =
    let x' = x `div` 3 * 3
        y' = y `div` 3 * 3
    in [(a, b) | a <- [x'..x' + 2], b <- [y'..y' + 2]]

isCorrectGroup :: String -> Bool
isValidRow, isValidColumn :: Int -> Sudoku -> Bool
isValidSquare :: (Int, Int) -> Sudoku -> Bool

isCorrectGroup = all (<=1) . tail . digits . read
isValidRow r sudoku = isCorrectGroup $ map (\i -> intToDigit $ sudoku ! (i, r)) [0..8]
isValidColumn c sudoku = isCorrectGroup $ map (\i -> intToDigit $ sudoku ! (c, i)) [0..8]
isValidSquare p sudoku = isCorrectGroup $ map (intToDigit . (sudoku!)) $ smallSquare p

isValidInsert :: (Int, Int) -> Sudoku -> Bool
isValidInsert (x, y) = isValidColumn x <&&> isValidRow y <&&> isValidSquare (x, y)

solveSudoku' :: [(Int, Int)] -> Sudoku -> [Sudoku]
solveSudoku' [] sudoku = [sudoku]
solveSudoku' (h:t) sudoku = if sudoku ! h /= 0
    then solveSudoku' t sudoku
    else [head solutions |
            val <- [1..9],
            let sudoku' = updateSudoku h val sudoku,
            isValidInsert h sudoku',
            let solutions = solveSudoku' t sudoku',
            not (null solutions)]
    
solveSudoku :: Sudoku -> Sudoku
solveSudoku = head . solveSudoku' sudokuPositions

getSum :: Sudoku -> Int
getSum = read . map intToDigit . take 3 . elems

euler096 :: IO String
euler096 = show . sum . map (getSum . solveSudoku) . splitSudokus . lines <$> getInput 96