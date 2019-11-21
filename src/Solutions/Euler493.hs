module Solutions.Euler493 where
import Utils.Numeric(binom)
import Control.Monad
import Text.Printf

rolls :: [[Integer]]
rolls = filter ((==20) . sum) $ replicateM 7 [0..10]

eways :: [Integer] -> Integer
eways l = toInteger (length $ filter (/=0) l) * product (map (binom 10) l)

ex :: Double
ex = fromIntegral (sum $ map eways rolls) / fromIntegral (binom 70 20)

euler493 :: IO String
euler493 = return $ printf "%.9d" ex