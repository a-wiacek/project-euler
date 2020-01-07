module Simulations.Euler237 where
import Utils.Operators((<&&>))
import Control.Monad.Reader
import Control.Monad.Trans.State.Strict
import Data.Array.Unboxed

type Position = (Int, Int)
type VisitedArray = UArray Position Bool -- False if not visited, True if visited
initVisitedArray :: Int -> VisitedArray
initVisitedArray n = listArray ((1, 1), (4, n)) $ True:repeat False

data SimulationEnv = SimulationEnv
    { visited :: VisitedArray
    , currentPosition :: Position
    }
    
initSimulationEnv :: Int -> SimulationEnv
initSimulationEnv n = SimulationEnv (initVisitedArray n) (1, 1)

visit :: Position -> SimulationEnv -> SimulationEnv
visit position oldEnv = SimulationEnv (visited oldEnv // [(position, True)]) position

type SimulationScore = Int
type Simulation a = StateT SimulationScore (Reader SimulationEnv) a

possibleDirections :: Simulation [Position]
possibleDirections = ask >>= \simulationEnv -> 
    let (x, y) = currentPosition simulationEnv
        n = snd $ snd $ bounds $ visited simulationEnv
        candidates = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        validPosition (x, y) = x >= 1 && x <= 4 && y >= 1 && y <= n
        notVisited pos = not $ visited simulationEnv ! pos
    in return $ filter (validPosition <&&> notVisited) candidates

validateRun :: Simulation ()
validateRun = ask >>= \simulation ->
    when (currentPosition simulation == (4, 1) && and (elems $ visited simulation)) (modify' succ)

oneStep :: Simulation ()
oneStep = possibleDirections >>= \positions ->
    if null positions
        then validateRun
        else forM_ positions $ \position -> local (visit position) oneStep

simulate :: Int -> Int
simulate n = runReader (execStateT oneStep 0) (initSimulationEnv n)

runSimulation237 :: IO ()
runSimulation237 = print $ map simulate [1..10]

