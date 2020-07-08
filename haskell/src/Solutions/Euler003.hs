module Solutions.Euler003 where
import Utils.NumberTheory(unPrime, factorize, runFactorization)
import Data.Maybe(fromJust)
import qualified Data.Map.Strict as M

euler003 :: IO String
euler003 = return $ show $ unPrime $ fst $ fromJust $ M.lookupMax
         $ runFactorization $ factorize (600851475143 :: Int)