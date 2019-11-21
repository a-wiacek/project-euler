module Solutions.Euler171 where
import Control.Monad
import Control.Monad.Memo
import Data.Array
import qualified Data.Map.Strict as Map

-- How long number has to be, what sum of squares of digits must it have, can we have leading zero?
type K = (Integer, Integer, Bool)
type V = (Integer, Integer) -- How many numbers are there and what is their sum?
type M = Map.Map K V

bound = 81 * 20 :: Integer

squares :: [Integer]
squares = map (^2) [0..]

sqrt' :: Integer -> Integer
sqrt' = floor . sqrt . fromIntegral

isSquare :: Integer -> Bool
isSquare n = n == sqrt' n ^ 2

initCaseTrue :: Integer -> (K, V)
initCaseTrue n = ((1, n, True), if n < 100 && isSquare n then (1, sqrt' n) else (0, 0))

initCaseFalse :: Integer -> (K, V)
initCaseFalse n = ((1, n, False), if n == 0 then (0, 0) else snd $ initCaseTrue n)

initMap :: M
initMap = Map.fromList $ map initCaseTrue [0..bound] ++ map initCaseFalse [0..bound]

plusV :: V -> V -> V
plusV (a, b) (c, d) = (a + c, b + d)
transform :: Integer -> Integer -> V -> V
transform d numLength (howMany, totalSum) = (howMany, totalSum + d * howMany * 10^(numLength - 1))

compute :: K -> Memo K V V
compute args@(numLength, digitsSquaresSum, zeroAllowed) = if digitsSquaresSum < 0
    then return (0, 0)
    else foldr plusV (0, 0) <$> forM [if zeroAllowed then 0 else 1..9]
            (\d -> transform d numLength <$> memo compute (numLength - 1, digitsSquaresSum - d^2, True))

doComputation :: Memo K V ()
doComputation = forM_ [2..20] $ \x -> forM_ [0..bound] $ \y -> memo compute (x, y, True) >> memo compute (x, y, False)

isValidI :: (K, V) -> Bool
isValidI ((a, b, _), _) = 1 <= a && a <= 20 && 0 <= b && b <= bound

arr :: Array K V
arr = array ((1, 0, False), (20, bound, True)) $ filter isValidI $ Map.assocs $ snd $ runMemo doComputation initMap

euler171 :: IO String
euler171 = return $ show $ (`mod` (10^9)) $ sum $ map (\y -> snd $ arr ! (20, y, True)) $ takeWhile (<=bound) squares