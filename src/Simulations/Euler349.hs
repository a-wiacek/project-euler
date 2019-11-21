module Simulations.Euler349 where
import qualified Data.Set as Set

-- Simulation of ant
-- Detailed explanation: http://oeis.org/A255938

data Direction = LEFT | RIGHT | UP | DOWN deriving (Show, Eq)
type Position = (Int, Int)
type Grid = Set.Set Position -- grid contains positions of black squares
type State = (Grid, Position, Direction)

clockwise :: Direction -> Direction
clockwise d | d == LEFT = UP
            | d == UP = RIGHT
            | d == RIGHT = DOWN
            | d == DOWN = LEFT
counterclockwise :: Direction -> Direction
counterclockwise d | d == LEFT = DOWN
                   | d == DOWN = RIGHT
                   | d == RIGHT = UP
                   | d == UP = LEFT

move :: Direction -> Position -> Position
move d (x, y) | d == LEFT = (x - 1, y)
              | d == RIGHT = (x + 1, y)
              | d == UP = (x, y + 1)
              | d == DOWN = (x, y - 1)
   
simulate :: Int -> State -> Grid
simulate n (grid, position, direction)
    | n == 0 = grid
    | position `Set.member` grid =
        let newDirection = counterclockwise direction
        in simulate (n - 1) (Set.delete position grid, move newDirection position, newDirection)
    | otherwise = 
        let newDirection = clockwise direction
        in simulate (n - 1) (Set.insert position grid, move newDirection position, newDirection)

initialState :: State
initialState = (Set.empty, (0, 0), UP)

runSimulation349 :: IO ()
runSimulation349 = print $ simulate 10000 initialState