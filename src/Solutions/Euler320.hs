module Solutions.Euler320 where
import Utils.NumberTheory(unPrime, primesUpTo, runFactorization, factorize)
import Control.Monad
import Control.Monad.ST
import Data.List
import Data.STRef
import qualified Data.Map.Strict as M

bound = 10^6
sumOfDigits n p = if n == 0 then 0 else n `rem` p + sumOfDigits (n `quot` p) p
primesInFactorial n p = (n - sumOfDigits n p) `quot` (p - 1)
checkFac n p k = if primesInFactorial n p >= k then n else checkFac (n - n `rem` p + p) p k
inverse p k = let n = k * (p - 1) in checkFac n p k
         
findLowest :: Int -> Int
findLowest n = maximum
             $ map (\p' -> let p = unPrime p' in inverse p $ primesInFactorial n p * 1234567890)
             $ primesUpTo n

compute :: Int
compute = runST $ do
    sum <- newSTRef 0
    max <- newSTRef 0
    forM_ [10..bound] $ \n -> do
        forM_ (M.keys $ runFactorization $ factorize n) $ \p' -> do
            let p = unPrime p'
            let count = primesInFactorial n p
            let inv = inverse p $ count * 1234567890
            max' <- readSTRef max
            when (max' < inv) $ writeSTRef max inv
        max' <- readSTRef max
        modifySTRef' sum (\x -> (x + max') `rem` 10^18)
    readSTRef sum

euler320 :: IO String
euler320 = return $ show compute 