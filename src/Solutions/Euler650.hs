module Solutions.Euler650 where
import Utils.Array
import Utils.NumberTheory
import Utils.Numeric(fastPowerMod)
import Data.Array
import qualified Data.Map.Strict as Map
import Data.Maybe

-- http://oeis.org/A001142
-- In entire file Factorization Int is used despite overflows:
-- we don't care about factoredNums.

bound = 20000 :: Int
p = 1000000007 :: Int

addP, mulP :: Int -> Int -> Int
addP a b = (a + b) `mod` p
mulP a b = (a * b) `mod` p
inv x = fromJust $ invertMod x p

f (q', a) = let q = unPrime q'
                z = fastPowerMod q (a + 1) p - 1
                x = inv (q - 1)
            in mulP z x
g f i = (f <> factPower (numfArr ! i) (fromIntegral $ i - 1)) `factMinus` (ffArr ! (i - 1))

sumOfDivisorsMod :: Factorization Int -> Int
sumOfDivisorsMod = foldr mulP 1 . map f . Map.toList . runFactorization

numfArr :: Array Int (Factorization Int)
numfArr = funArray 1 (bound + 1) factorize

ffArr :: Array Int (Factorization Int)
ffArr = funArray 1 bound factorialsFactorization

factorialsFactorization :: Int -> Factorization Int
factorialsFactorization 1 = mempty
factorialsFactorization n = ffArr ! (n - 1) <> numfArr ! n

funS n = foldr addP 0 $ map sumOfDivisorsMod $ scanl g mempty [2..n]

euler650 :: IO String
euler650 = return $ show $ funS bound