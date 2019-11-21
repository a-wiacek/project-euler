module Solutions.Euler217 where
import Solutions.Euler171(K, V, M, plusV, transform)
import Control.Monad
import Control.Monad.Memo
import Data.Array
import qualified Data.Map.Strict as Map

-- How long number has to be, what sum of digits must it have, can we have leading zero?
-- type K = (Integer, Integer, Bool)
-- type V = (Integer, Integer) -- How many numbers are there and what is their sum?
-- type M = Map.Map K V

initMap :: M
initMap = Map.fromList $ [((1, x, False), (if x > 0 then 1 else 0, x)) | x <- [0..9]] ++ [((1, x, True), (1, x)) | x <- [0..9]]

compute :: K -> Memo K V V
compute args@(numLength, digitsSum, zeroAllowed) =
    if digitsSum < 0 || numLength < 1
        then return (0, 0)
        else foldr plusV (0, 0) <$> forM [if zeroAllowed then 0 else 1..9]
                (\d -> transform d numLength <$> memo compute (numLength - 1, digitsSum - d, True))

doComputation :: Memo K V ()
doComputation = forM_ [2..23] $ \x -> forM_ [0..9 * x] $ \y -> memo compute (x, y, True) >> memo compute (x, y, False)

isValidI :: (K, V) -> Bool
isValidI ((a, b, _), _) = 1 <= a && a <= 23 && 0 <= b && b <= 207

arr :: Array K V
arr = array ((1, 0, False), (23, 207, True)) $ filter isValidI $ Map.assocs $ snd $ runMemo doComputation initMap

balanced' :: Integer -> Integer -> Integer
balanced' n digitsSum
    | even n = let (upperCount, upper) = arr ! (n `div` 2, digitsSum, False)
                   (lowerCount, lower) = arr ! (n `div` 2, digitsSum, True)
               in lowerCount * upper * 10^(n `div` 2) + upperCount * lower
    | otherwise = let (upperCount, upper) = arr ! (n `div` 2, digitsSum, False)
                      (lowerCount, lower) = arr ! (n `div` 2, digitsSum, True)
                   in 10 * (lowerCount * upper * 10^(n `div` 2 + 1) + upperCount * lower) + lowerCount * upperCount * 45 * 10^(n `div` 2)

balanced :: Integer -> Integer
balanced n = if n == 1 then 45 else sum $ map (balanced' n) [1..9 * (n `div` 2)]

euler217 :: IO String
euler217 = return $ show $ (`mod` (3^15)) $ sum $ map balanced [1..47]