module Solutions.Euler003 where
import Utils.NumberTheory(factorize, runFactorization)
import Data.Maybe(fromJust)
import qualified Data.Map.Strict as M

euler003 :: IO String
euler003 = return $ show $ fst $ fromJust $ M.lookupMax $ runFactorization $ factorize (600851475143 :: Int)