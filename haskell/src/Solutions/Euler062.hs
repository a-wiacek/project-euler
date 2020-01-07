module Solutions.Euler062 where
import Utils.Operators((<:>))
import Data.List

compairs (a, b) (c, d) = case compare b d of
    EQ -> compare a c
    x -> x

compute = (^3) . foldr (min . fst . head) 9999
        . filter ((==5) . length) . groupBy (\x y -> snd x == snd y)
        . sortBy compairs . map (id <:> (sort . show . (^3))) $ [400..20000]
    
euler062 :: IO String
euler062 = return $ show compute