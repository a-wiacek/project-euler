{-# LANGUAGE Strict #-}
module Solutions.Euler623 where
import Utils.Memo

pplus x y = (x + y) `mod` (10^9 + 7)
psum = foldr pplus 0

-- Counts lambda terms with exact length len and vars unbounded variables
exactLambdaCount :: (Int, Int) -> StrictMemo (Int, Int) Int Int
exactLambdaCount (len, vars)
    | len <= 0 = return 0
    | len == 1 = return vars
    | otherwise = do
        application <- fmap psum $ forM [1..len - 3] $ \m ->
            (*) <$> memo exactLambdaCount (m, vars)
                <*> memo exactLambdaCount (len - 2 - m, vars)
        abstraction <- memo exactLambdaCount (len - 5, vars + 1)
        return $ pplus application abstraction

lambdaCount :: Int -> StrictMemo (Int, Int) Int Int
lambdaCount n = psum <$> forM [6..n] (\l -> memo exactLambdaCount (l, 0))

runLambdaCount :: Int -> Int
runLambdaCount = startEvalMemo . lambdaCount

euler623 :: IO String
euler623 = return $ show $ runLambdaCount 2000