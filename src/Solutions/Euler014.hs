module Solutions.Euler014 where
import Control.Monad.Memo

type M = Memo Int Int

compute :: Int -> M Int
compute 1 = return 0
compute n = succ <$> memo compute (if even n then n `div` 2 else 3 * n + 1)

values :: M [(Int, Int)]
values = forM [2..999999] $ \x -> memo compute x >>= \c -> return (x, c)

process :: M Int
process = fst . foldr1 max' <$> values

max' :: (Int, Int) -> (Int, Int) -> (Int, Int)
max' (k1, v1) (k2, v2) = if v1 > v2 then (k1, v1) else (k2, v2)

euler014 :: IO String
euler014 = return $ show $ startEvalMemo process