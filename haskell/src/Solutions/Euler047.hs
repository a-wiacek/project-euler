module Solutions.Euler047 where
import qualified Data.Map.Strict as M
import Utils.List(uniques)
import Utils.NumberTheory(runFactorization, factorize)

fourUnique n
    | fs n /= 4 = fourUnique $ n + 1
    | fs (n + 1) /= 4 = fourUnique $ n + 2
    | fs (n + 2) /= 4 = fourUnique $ n + 3
    | fs (n + 3) /= 4 = fourUnique $ n + 4
    | otherwise = n
    where fs = M.size . runFactorization . factorize

euler047 :: IO String
euler047 = return $ show $ fourUnique (644 :: Int)