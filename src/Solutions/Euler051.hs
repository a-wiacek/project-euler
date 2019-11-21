module Solutions.Euler051 where
import Utils.List(powerset)
import Utils.NumberTheory(primesArrayUpTo)
import Control.Monad
import Data.Array.Unboxed((!))
import Data.Char
import Data.Maybe

nmax = 1000000
primes = primesArrayUpTo nmax

replaceElems :: String -> Char -> [Int] -> String
replaceElems oldList newElem positions = r oldList positions "" 0 where
    r :: String -> [Int] -> String -> Int -> String
    r old pos acc ctr 
        | null old || null pos = reverse (reverse old ++ acc)
        | head pos == ctr = r (tail old) (tail pos) (newElem:acc) (ctr + 1)
        | otherwise = r (tail old) pos (head old:acc) (ctr + 1)

replaceSubset :: Int -> [Int] -> (Int, Int)
replaceSubset n subset = (foldr min nmax l, length l) where
    l = do i <- [0..9]
           guard $ i /= 0 || 0 `notElem` subset
           let n' = read $ replaceElems (show n) (intToDigit i) subset
           guard $ primes ! n'
           return n'

replacePrimes :: Int -> Maybe Int
replacePrimes n = if ans < nmax then Just ans else Nothing where
    ans = foldr (min . fst) nmax
              $ filter ((==8) . snd)
              $ map (replaceSubset n) 
              $ filter ((==3) . length) $ init $ powerset [0..length (show n) - 1]

loop :: Int -> Int
loop n = fromMaybe (loop $ n + 1) $ replacePrimes n

euler051 :: IO String
euler051 = return $ show $ loop 100000