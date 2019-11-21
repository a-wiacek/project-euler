module Solutions.Euler101 where
import Utils.List(deleteAt)
import Utils.Operators((<:>))

-- Lagrange interpolation polynomial, Lagrange basis
lagrangeBase :: Int -> [Double] -> Double -> Double
lagrangeBase n xs x = let xs' = map (flip (-)) $ deleteAt n xs in
    product (map ($ x) xs') / product (map ($ xs !! n) xs')

lagrangePoly :: [(Int, Double)] -> Double -> Double
lagrangePoly ls x =
    let xs = map (fromIntegral . fst) ls :: [Double]
        ys = map snd ls :: [Double]
    in sum $ map (\(y, i) -> y * lagrangeBase i xs x) $ zip ys [0..]

u :: Double -> Double
u n = let sign = 1:(-1):sign in sum $ zipWith (*) sign $ map (n^) [0..10]

value :: Double -> Int
value n = floor $ lagrangePoly (map (floor <:> u) [1..n]) (n + 1)

euler101 :: IO String
euler101 = return $ show $ sum $ map value [1..10]