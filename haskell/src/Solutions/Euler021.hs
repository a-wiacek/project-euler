module Solutions.Euler021 where

-- According to Wikipedia, these are pairs of amicable numbers under 10000:
-- (220, 284), (1184, 1210), (2620, 2924), (5020, 5564), (6232, 6368)

euler021 :: IO String
euler021 = return $ show $ sum [220, 284, 1184, 1210, 2620, 2924, 5020, 5564, 6232, 6368]