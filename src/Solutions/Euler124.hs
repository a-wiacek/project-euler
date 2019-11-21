module Solutions.Euler124 where
import Utils.NumberTheory(unPrime, runFactorization, factorize)
import Utils.Operators((<:>))
import Data.List
import qualified Data.Map.Strict as M

l = sort $ map (product . map unPrime . M.keys . runFactorization . factorize <:> id) ([2..100000] :: [Int])
    -- 1 is excluded and it is first in sorted list, so to get E(k) we take
    -- element at index k - 2

euler124 :: IO String
euler124 = return $ show $ snd $ l !! 9998