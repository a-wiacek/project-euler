module Solutions.Euler084 where
import Utils.Array(modifyArray)
import qualified Utils.Queue as Q
import Control.Monad
import Control.Monad.ST
import Control.Monad.Trans
import Control.Monad.Trans.State.Strict
import Data.Array.ST
import Data.List(sortBy)
import Data.STRef
import System.Random
import Text.Printf


newtype Square = Square Int deriving (Eq, Ord, Ix)
instance Show Square where show (Square n) = printf "%02d" n

squareGO = Square 0
squareR1 = Square 5
squareJAIL = Square 10
squareC1 = Square 11
squareE3 = Square 24
squareG2J = Square 30
squareH2 = Square 39
squaresCC = [Square 2, Square 17, Square 33]
squaresCH = [Square 7, Square 22, Square 36]
nextR (Square n) | n > 5 && n < 15 = Square 15
                 | n > 15 && n < 25 = Square 25
                 | n > 25 && n < 35 = Square 35
                 | otherwise = Square 5
nextU (Square n) | n > 12 && n < 28 = Square 28
                 | otherwise = Square 12

applyStep :: Int -> Square -> Square
applyStep step (Square n) =
    let newSquare = Square $ (n + step) `mod` 40
    in if newSquare == squareG2J then squareJAIL else newSquare


type VisitedCounter s = STUArray s Square Int

getAnswer :: [(Square, Int)] -> String
getAnswer assocs =
    let [(max1, _), (max2, _), (max3, _)] = take 3 $ sortBy (\(_, a) (_, b) -> compare b a) assocs
    in show max1 ++ show max2 ++ show max3


data Card = Go | Jail | C1 | E3 | H2 | R1 | NextR | NextU | Back3 | Null
communityCards = [Go, Jail] ++ replicate 14 Null
chanceCards = [Go, Jail, C1, E3, H2, R1, NextR, NextR, NextU, Back3] ++ replicate 6 Null
newtype Deck = Deck (Q.Queue Card)
mkDeck :: [Card] -> Deck
mkDeck cards = Deck (Q.fromList cards)

takeCard :: Deck -> (Card, Deck)
takeCard (Deck q) =
    let (Just card, q') = Q.dequeue q
    in (card, Deck $ Q.enqueue card q')


data GameState s = GameState
    { position :: Square
    , visitedCounter :: VisitedCounter s
    , doubles :: Int
    , communityDeck :: Deck
    , chanceDeck :: Deck
    , gen :: StdGen
    }

initGameState :: StdGen -> VisitedCounter s -> GameState s
initGameState g counter =
    let (communityCardsShuffled, g') = shuffle communityCards g
        (chanceCardsShuffled, g'') = shuffle chanceCards g'
    in GameState squareGO counter 0
                 (mkDeck communityCardsShuffled)
                 (mkDeck chanceCardsShuffled)
                 g''


type SimulationState s a = StateT (GameState s) (ST s) a
takeCommunityCard :: SimulationState s Card
takeCommunityCard = get >>= \state ->
    let (card, newCommunityDeck) = takeCard (communityDeck state)
    in put (state { communityDeck = newCommunityDeck }) >> return card

applyCommunityCard :: Square -> SimulationState s ()
applyCommunityCard sq = takeCommunityCard >>= \card -> goToSquare $ case card of
    Go -> squareGO
    Jail -> squareJAIL
    Null -> sq

takeChanceCard :: SimulationState s Card
takeChanceCard = get >>= \state ->
    let (card, newChanceDeck) = takeCard (chanceDeck state)
    in put (state { chanceDeck = newChanceDeck }) >> return card

applyChanceCard :: Square -> SimulationState s ()
applyChanceCard sq = takeChanceCard >>= \card -> goToSquare $ case card of
    Go -> squareGO
    Jail -> squareJAIL
    C1 -> squareC1
    E3 -> squareE3
    H2 -> squareH2
    R1 -> squareR1
    NextR -> nextR sq
    NextU -> nextU sq
    Back3 -> applyStep (-3) sq
    Null -> sq

rollDices :: SimulationState s Int
rollDices = get >>= \state ->
    let (roll1, gen') = randomR (1, 4) $ gen state
        (roll2, gen'') = randomR (1, 4) gen'
        doubles' = if roll1 == roll2 then doubles state + 1 else 0
    in put (state { gen = gen'', doubles = doubles' }) >> return (roll1 + roll2)

getNextPosition :: SimulationState s Square
getNextPosition = do
    step <- rollDices
    dbls <- gets doubles
    if dbls == 3
        then return squareJAIL
        else applyStep step <$> gets position

goToSquare :: Square -> SimulationState s ()
goToSquare sq = do
    state <- get
    lift $ modifyArray (visitedCounter state) sq succ
    put state { position = sq }

movePawn :: Square -> SimulationState s ()
movePawn newPosition
    | newPosition `elem` squaresCC = applyCommunityCard newPosition
    | newPosition `elem` squaresCH = applyChanceCard newPosition
    | otherwise = goToSquare newPosition

simulateGame :: Int -> SimulationState s ()
simulateGame moves = replicateM_ moves (getNextPosition >>= movePawn)


makeArray :: [a] -> ST s (STArray s Int a)
makeArray list = newListArray (1, length list) list

shuffle :: [a] -> StdGen -> ([a], StdGen)
shuffle list g = runST $ do
    gen <- newSTRef g
    arr <- makeArray list
    forM_ [length list..2] $ \i -> do
        (j, g') <- randomR (1, i) <$> readSTRef gen
        writeSTRef gen g'
        tmp <- readArray arr i
        readArray arr j >>= writeArray arr i
        writeArray arr j tmp
    shuffledList <- getElems arr
    g' <- readSTRef gen
    return (shuffledList, g')

totalMoves = 2000000 :: Int
evalGame :: StdGen -> String
evalGame g = runST $ do
    arr <- newArray (squareGO, squareH2) 0 :: ST s (VisitedCounter s)
    evalStateT (simulateGame totalMoves) (initGameState g arr)
    getAnswer <$> getAssocs arr


euler084 :: IO String
euler084 = evalGame <$> newStdGen