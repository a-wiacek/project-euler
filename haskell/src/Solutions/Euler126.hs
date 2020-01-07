module Solutions.Euler126 where
import Data.List
import Utils.List(reduceList)

cubes :: Int -> Int -> Int -> Int -> Int
cubes x y z n = 2 * (x * y + y * z + z * x) + 4 * (x + y + z + n - 2) * (n - 1)

bound = 25000 :: Int
allValues = [cubes x y z n | x <- takeWhile (\t -> cubes t t t 1 < bound) [1..]
                           , y <- takeWhile (\t -> cubes x t t 1 < bound) [x..]
                           , z <- takeWhile (\t -> cubes x y t 1 < bound) [y..]
                           , n <- takeWhile (\t -> cubes x y z t < bound) [1..]]
sortedValues = map reduceList $ group $ sort allValues
funX n = (\(Just (x, _)) -> x) $ find ((==n) . snd) sortedValues

euler126 :: IO String
euler126 = return $ show $ funX 1000