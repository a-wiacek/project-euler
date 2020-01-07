module Solutions.Euler303 where
import Utils.NumberTheory(divisors)
import Utils.Operators((<:>))
import Control.Applicative((<|>))
import Data.Array
import Data.List(find)
import Data.Maybe(fromJust)
import Data.Word

-- Let g(n) = f(n) / n. Two important observations that allow us to shorten computations:
-- g(10 * n) = g(n)
-- if g(n) = a * k, then g(n * k) = a (assuming all numbers are integers)

-- Computing g(9999) took much more time than computing all other 9999 values of g.

type Unit = Word16

suffixes :: Int -> [Integer]
suffixes 0 = [1, 2]
suffixes k = [10 * x + y | x <- suffixes (k - 1), y <- [0, 1, 2]]

rawCompute :: Unit -> Maybe Integer
rawCompute n = (`div` n') <$> funF' 0 where
    n' = toInteger n
    funF' :: Int -> Maybe Integer
    funF' k = find (\u -> u `mod` n' == 0) (suffixes k) <|> funF' (k + 1)
    
compute :: Unit -> Integer
compute bound =
    let funG :: Array Unit Integer
        funG = array (1, bound) $ map (id <:> computeG) [1..bound]
        computeG :: Unit -> Integer
        computeG n
            | n == 1 = 1
            | n `mod` 10 == 0 = funG ! (n `div` 10)
            | otherwise = fromJust $ foldr1 (<|>) $ rawCompute n : map funGLow properDivisors
            where properDivisors :: [Unit]
                  properDivisors = map fromIntegral $ init $ tail $ divisors $ fromIntegral n
                  funGLow :: Unit -> Maybe Integer
                  funGLow k = let k' = toInteger k
                                  g = funG ! (n `div` k)
                                  in if g `mod` k' == 0
                                      then Just (g `div` k')
                                      else Nothing
    in sum $ elems funG

euler303 :: IO String
euler303 = return $ show $ compute 10000