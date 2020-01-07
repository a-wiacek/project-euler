module Main.Simulate(simulate) where
import Simulations.Euler237(runSimulation237)
import Simulations.Euler306(runSimulation306)
import Simulations.Euler335(runSimulation335)
import Simulations.Euler349(runSimulation349)
import Text.Printf

getSimulation :: Int -> Maybe (IO ())
getSimulation n
    | n == 237 = Just runSimulation237
    | n == 306 = Just runSimulation306
    | n == 335 = Just runSimulation335
    | n == 349 = Just runSimulation349
    | otherwise = Nothing

simulate :: Int -> IO ()
simulate problemNo = case getSimulation problemNo of
    Nothing -> printf "Could not find simulation associated with problem %d\n" problemNo
    Just simulation -> printf "Running simulation associated with problem %d\n" problemNo >> simulation