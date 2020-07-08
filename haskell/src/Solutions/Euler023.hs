module Solutions.Euler023 where
import qualified Data.Set as Set

listPlus l1 l2 = (+) <$> l1 <*> l2

bound = 28123 :: Int
divisors :: Int -> [Int]
divisors n = filter ((==0) . mod n) [1..n - 1]
abundant :: [Int]
abundant = filter (\n -> sum (divisors n) > n) [1..bound]

euler023 :: IO String
euler023 = return $ show $ sum $ Set.toList
         $ Set.fromList [1..bound] Set.\\ Set.fromList (listPlus abundant abundant)