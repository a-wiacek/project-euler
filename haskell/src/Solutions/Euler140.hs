module Solutions.Euler140 where
import Solutions.Euler137(compair, uniq)
import Utils.List(uniquesBy)
import Data.List(sortBy)

recursives = [\(x, y) -> (-9 * x - 4 * y - 14, -20 * x - 9 * y - 28),
              \(x, y) -> (-9 * x + 4 * y - 14, 20 * x - 9 * y + 28)]

basePairs = [(2, -7), (0, -1), (0, 1), (-4, 5), (-3, 2), (-3, 2)]

solutions = concat $ do
    fn <- recursives
    take 30 . iterate fn <$> basePairs

euler140 :: IO String
euler140 = return $ show $ sum $ map fst $ take 30 $ uniquesBy uniq $ sortBy compair $ filter ((>0) . fst) solutions