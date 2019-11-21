module Solutions.Euler122 where
import Control.Monad.State.Strict
import Data.Maybe
import qualified Data.IntMap.Strict as Map
import qualified Data.Set as Set

type S = Set.Set Int
type I = Map.IntMap Int
type SS = Set.Set S
bound = 200

multiply :: S -> [S]
multiply set =
    let l = Set.toList set
    in [Set.insert (a + b) set | a <- l, b <- l, a <= b, a + b <= bound, not (Set.member (a + b) set)]

transformSets :: Int -> SS -> SS
transformSets k = Set.unions . map (Set.fromList . filter (heuristics k) . multiply) . Set.toList

heuristics k set = k < 6 || Set.findMax set > 4 * k

data CState = CState
    { step :: Int
    , presentSets :: SS
    , values :: I
    , instanceSize :: Int }

type Computation a = State CState a
updateValueAt :: Int -> Computation ()
updateValueAt n = get >>= \state ->
    let v = values state
    in when (isNothing (v Map.!? n) && any (Set.member n) (presentSets state))
            $ put state { values = Map.insert n (step state) v }

updateValues :: Computation ()
updateValues = gets instanceSize >>= \s -> forM_ [1..s] updateValueAt

updatePresentSets :: Computation ()
updatePresentSets = get >>= \state -> put state { presentSets = transformSets (step state) (presentSets state) }

updateStep :: Computation ()
updateStep = get >>= \state -> put state { step = step state + 1 }

makeOneStep :: Computation [Int]
makeOneStep = updateValues >> get >>= \state -> if Map.size (values state) < instanceSize state && step state /= 10
        then updatePresentSets >> updateStep >> makeOneStep
        else return $ map snd $ Map.toList $ values state

initState :: Int -> CState
initState = CState 0 (Set.singleton $ Set.singleton 1) Map.empty

l :: [Int]
l = evalState makeOneStep $ initState bound

euler122 :: IO String
euler122 = return $ show $ sum l + (bound - length l) * 11