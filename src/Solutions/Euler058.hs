module Solutions.Euler058 where
import Utils.Monad(whenM)
import Utils.NumberTheory(primesUpTo, isPrime)
import Control.Monad
import Control.Monad.ST
import Data.STRef

result :: Int
result = runST $ do
    primes <- newSTRef (0 :: Int)
    total <- newSTRef (1 :: Int)
    step <- newSTRef (2 :: Int)
    curr <- newSTRef (1 :: Int)
    let loop = do s <- readSTRef step
                  replicateM_ 4 $ do
                      modifySTRef curr (+s)
                      modifySTRef total succ
                      whenM (isPrime <$> readSTRef curr) (modifySTRef primes succ)
                  p <- readSTRef primes
                  t <- readSTRef total
                  if 10 * p < t
                      then succ <$> readSTRef step
                      else modifySTRef step (+2) >> loop
    loop

euler058 :: IO String
euler058 = return $ show result