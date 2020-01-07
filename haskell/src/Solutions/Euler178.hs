module Solutions.Euler178 where
import Control.Monad
import Control.Monad.Memo
import qualified Data.Map.Strict as Map

bound = 40

type K = (Int, Int, Int, Int)
type V = Int
compute :: K -> Memo K V V
compute args@(lowest, highest, left, current)
    | current < 0 || current > 9 = return 0
    | left == 0 = return $ if lowest == 0 && highest == 9 then 1 else 0
    | otherwise = let d1 = current - 1
                      d2 = current + 1
                  in (+) <$> memo compute (min lowest d1, highest, left - 1, d1)
                         <*> memo compute (lowest, max highest d2, left - 1, d2)

doComputation :: Memo K V ()
doComputation = forM_ [10..bound] $ \n -> forM_ [1..9] $ \x -> memo compute (x, x, n - 1, x)

arr :: Map.Map (Int, Int, Int, Int) Int
arr = snd $ runMemo doComputation $ Map.fromList [((0, 9, 0, x), 1) | x <- [0..9]]

f :: Int -> Int
f n = sum $ map (\x -> arr Map.! (x, x, n - 1, x)) [1..9]

euler178 :: IO String
euler178 = return $ show $ sum $ map f [10..bound]