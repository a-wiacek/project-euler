module Solutions.Euler138 where

-- Let b = 2x, then
-- 5x^2 (+-) 4x + 1 = L^2
-- Plug it into https://www.alpertron.com.ar/QUAD.HTM

sols (x, y) = abs y : sols (-9 * x - 4 * y - 4, -20 * x - 9 * y - 8)

euler138 :: IO String
euler138 = return $ show $ sum $ take 12 $ tail $ sols (0, 1)