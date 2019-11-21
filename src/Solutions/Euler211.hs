module Solutions.Euler211 where
import Utils.Array(modifyArray)
import Utils.NumberTheory(divisors)
import Utils.Numeric(isSquare)
import Control.Monad
import Data.Array.ST
import Data.Array.Unboxed

compute :: Int -> Int
compute n =
    let n' = floor $ sqrt $ fromIntegral $ n - 1
        divArray = runSTUArray $ do
            arr <- newArray (1, n - 1) 0
            forM_ [1..n'] $ \d -> let d2 = d * d in do
                modifyArray arr d2 (+d2)
                forM_ [d2 + d, d2 + d + d..n - 1] $ \b ->
                      let bd = b `div` d in modifyArray arr b (+ (d * d + bd * bd))
            return arr
    in sum $ map fst $ filter (isSquare . snd) $ assocs divArray

euler211 :: IO String
euler211 = return $ show $ compute 64000000