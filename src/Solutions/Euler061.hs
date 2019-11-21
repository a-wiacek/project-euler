module Solutions.Euler061 where
import Data.List

fourDigits :: [Int] -> [Int]
fourDigits = takeWhile (<10000) . dropWhile (<1000)
polysts :: Int -> [Int]
polysts x = fourDigits $ map fn [1..] where
    fn n = case x of
        3 -> n * (n + 1) `div` 2
        4 -> n^2
        5 -> n * (3 * n - 1) `div` 2
        6 -> n * (2 * n - 1)
        7 -> n * (5 * n - 3) `div` 2
        8 -> n * (3 * n - 2)

begin = take 2 . show
end = drop 2 . show

compute = head [n1 + n2 + n3 + n4 + n5 + n6 |
    n1 <- polysts 3,
    [o1, o2, o3, o4, o5] <- permutations [4..8],
    n2 <- polysts o1,
    end n1 == begin n2,
    n3 <- polysts o2,
    end n2 == begin n3,
    n4 <- polysts o3,
    end n3 == begin n4,
    n5 <- polysts o4,
    end n4 == begin n5,
    n6 <- polysts o5,
    end n5 == begin n6,
    end n6 == begin n1]
    
euler061 :: IO String
euler061 = return $ show compute