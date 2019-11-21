module Solutions.Euler054 where
import Utils.Input(getInput)
import Data.Char
import Data.List
import Data.Ord
import Data.Maybe

type Value = Int -- 11 = J, 12 = Q, 13 = K, 14 = A
jack = 11
queen = 12
king = 13
ace = 14
data Suit = Diamond | Club | Heart | Spade deriving (Eq, Show)
data Card = Card {value :: Value, suit :: Suit} deriving (Eq, Show)
instance Ord Card where compare c1 c2 = compare (value c1) (value c2)

parseCard :: String -> Card
parseCard [v, s] = Card v' s' where
    v' = case v of
        'T' -> 10
        'J' -> 11
        'Q' -> 12
        'K' -> 13
        'A' -> 14
        _ -> digitToInt v
    s' = case s of
        'D' -> Diamond
        'C' -> Club
        'H' -> Heart
        'S' -> Spade

parseHand :: String -> [Card]
parseHand handStr = [c1, c2, c3, c4, c5] where
    c1 = parseCard $ take 2 handStr
    c2 = parseCard $ (take 2 . drop 3) handStr
    c3 = parseCard $ (take 2 . drop 6) handStr
    c4 = parseCard $ (take 2 . drop 9) handStr
    c5 = parseCard $ (take 2 . drop 12) handStr

parseTwoHands :: String -> ([Card], [Card])
parseTwoHands handsStr = (hand1, hand2) where
    hand1 = parseHand handsStr
    hand2 = parseHand $ drop 15 handsStr

data HandRank
    = HighCard -- remaining: all 5 cards sorted
    | OnePair Value -- remaining: 3 cards sorted
    | TwoPairs Value Value -- v1 > v2, remaining: last card
    | Three Value -- remaining: 2 cards sorted
    | Straight Value -- highest value
    | Flush -- remaining: all 5 cards
    | FullHouse Value Value -- 3 + 2
    | Four Value -- remaining: last card
    | StraightFlush Value -- highest value
    | RoyalFlush
    deriving (Eq, Ord, Show)

data Hand = Hand { handRank :: HandRank, remaining :: [Value] }
    deriving (Eq, Ord, Show)

straight :: [Card] -> Bool
straight [card1, card2, card3, card4, card5] =
    let h = value card1
    in value card2 == h - 1 && value card3 == h - 2 && value card4 == h - 3 && value card5 == h - 4

flush :: [Card] -> Bool
flush hand = let s = suit (head hand) in all (\c -> suit c == s) hand

four :: [Card] -> Maybe Hand
four [card1, card2, card3, card4, card5]
    | value card2 /= value card3 || value card3 /= value card4 = Nothing
    | value card1 == value card3 = Just $ Hand (Four (value card3)) [value card5]
    | value card5 == value card3 = Just $ Hand (Four (value card3)) [value card1]
    | otherwise = Nothing

pos3 :: [Card] -> Maybe Int
pos3 [card1, card2, card3, card4, card5] -- 3rd card must be in three of a kind
    | p1 && p2 = Just 1
    | p2 && p4 = Just 2
    | p4 && p5 = Just 3
    | otherwise = Nothing
    where p1 = value card1 == value card3
          p2 = value card2 == value card3
          p4 = value card4 == value card3
          p5 = value card5 == value card3

evalHand :: [Card] -> Hand
evalHand unsortedHand =
    let hand@[card1, card2, card3, card4, card5] = sortOn Data.Ord.Down unsortedHand :: [Card] in -- sorted from highest to lowest
    case (straight hand, flush hand) of
        (True, True) -> if value card1 == ace
            then Hand RoyalFlush []
            else Hand (StraightFlush $ value card1) []
        (True, False) -> Hand (Straight $ value card1) []
        (False, True) -> Hand Flush $ map value hand
        (False, False) -> flip fromMaybe (four hand) $
            case pos3 hand of
                Just 2 -> Hand (Three $ value card2) (map value [card1, card5])
                Just 1 -> if value card4 == value card5
                    then Hand (FullHouse (value card1) (value card4)) []
                    else Hand (Three $ value card1) (map value [card4, card5])
                Just 3 -> if value card1 == value card2
                    then Hand (FullHouse (value card3) (value card1)) []
                    else Hand (Three $ value card3) (map value [card1, card2])
                Nothing ->
                    let p1 = value card1 == value card2
                        p2 = value card2 == value card3
                        p3 = value card3 == value card4
                        p4 = value card4 == value card5 in
                        case (p1, p2, p3, p4) of
                            (True, False, True, False) -> Hand (TwoPairs (value card1) (value card3)) [value card5]
                            (False, True, False, True) -> Hand (TwoPairs (value card2) (value card4)) [value card1]
                            (True, False, False, True) -> Hand (TwoPairs (value card1) (value card4)) [value card3]
                            (True, False, False, False) -> Hand (OnePair (value card1)) (map value [card3, card4, card5])
                            (False, True, False, False) -> Hand (OnePair (value card2)) (map value [card1, card4, card5])
                            (False, False, True, False) -> Hand (OnePair (value card3)) (map value [card1, card2, card5])
                            (False, False, False, True) -> Hand (OnePair (value card4)) (map value [card1, card2, card3])
                            _ -> Hand HighCard (map value hand)

evalLine :: String -> (Hand, Hand)
evalLine = (\(a, b) -> (evalHand a, evalHand b)) . parseTwoHands

euler054 :: IO String
euler054 = show . length . filter (uncurry (>)) . map evalLine . lines <$> getInput 54