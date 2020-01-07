module Solutions.Euler205 where
import Utils.List(reduceList)
import Control.Monad
import Data.List
import Text.Printf

process = map reduceList . group . sort . map sum
peter = process $ replicateM 9 [1..4]
colin = process $ replicateM 6 [1..6]

wins = sum [ptimes * ctimes | (pscore, ptimes) <- peter, (cscore, ctimes) <- colin, pscore > cscore]

euler205 :: IO String
euler205 = return $ printf "%.7f" $ (fromIntegral wins :: Double) / (4^9 * 6^6)