module Solutions.Euler342 where
import Utils.NumberTheory
import qualified Data.Map.Strict as Map
import Data.List(inits)

-- \varphi(n^2) = \varphi(n) * n
totientOfSqIsCube :: Factorization Integer -> Bool
totientOfSqIsCube f = all (\x -> x `mod` 3 == 0) $ runFactorization $ Map.foldrWithKey
    (\p e f' -> f' <> primePowerFact p (e - 1) <> factorize (unPrime p - 1))
    f (runFactorization f)

lessThan :: Factorization Integer -> Integer -> [Prime Integer] -> [Factorization Integer]
lessThan prod bound [] = [prod | factoredNum prod < bound]
lessThan prod bound (p:ps)
    | fNum >= bound = []
    | fNum * unPrime p >= bound = [prod]
    | otherwise = lessThan (primePowerFact p 1 <> prod) bound (p:ps) ++ lessThan prod bound ps
    where fNum = factoredNum prod

computeWithPrime :: Integer -> Prime Integer -> [Prime Integer] -> Integer
computeWithPrime bound p ps =
    let maxBk = takeWhile (\x -> unPrime p ^ x < bound) [2, 5..] :: [Word]
        initFactors = map (primePowerFact p) maxBk :: [Factorization Integer]
        computePart :: Factorization Integer -> [Factorization Integer]
        computePart initFactor = lessThan initFactor bound ps
        candidates = concatMap computePart initFactors :: [Factorization Integer]
    in sum $ map factoredNum $ filter totientOfSqIsCube candidates 

compute :: Integer -> Integer
compute bound = sum $ map (\l -> computeWithPrime bound (last l) (init l))
              $ tail $ inits $ primesUpTo $ floor $ sqrt $ fromIntegral bound

euler342 :: IO String
euler342 = return $ show $ compute $ 10^10