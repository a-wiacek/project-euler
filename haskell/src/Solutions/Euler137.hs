module Solutions.Euler137 where
import Utils.List(uniquesBy)
import Data.List(sortBy)

recursives = [\(x, y) -> (-9 * x - 4 * y - 2, -20 * x - 9 * y - 4),
              \(x, y) -> (-9 * x + 4 * y - 2, 20 * x - 9 * y + 4)]

basePairs = [(0, 1), (2, 5), (0, -1), (-1, 2), (-1, -2)]

compair (a, b) (c, d) = compare a c
uniq (a, b) (c, d) = a == c

solutions = concat $ do
    fn <- recursives
    take 30 . iterate fn <$> basePairs

euler137 :: IO String
euler137 = return $ show $ map fst (take 30 $ uniquesBy uniq $ sortBy compair $ filter ((>0) . fst) solutions) !! 14