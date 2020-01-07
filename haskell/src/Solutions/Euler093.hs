module Solutions.Euler093 where
import Utils.Operators((<:>))
import Control.Monad
import Data.Char
import Data.List
import Data.Maybe

data MatOp = Add | Sub | Mul | Div
data Exp a = EOp MatOp (Exp a) (Exp a) | EVal a

evalExp' :: Exp Double -> Maybe Double
evalExp' (EOp Add e1 e2) = (+) <$> evalExp' e1 <*> evalExp' e2
evalExp' (EOp Sub e1 e2) = (-) <$> evalExp' e1 <*> evalExp' e2
evalExp' (EOp Mul e1 e2) = (*) <$> evalExp' e1 <*> evalExp' e2
evalExp' (EOp Div e1 e2) = case (/) <$> evalExp' e1 <*> evalExp' e2 of
    Just x -> if isInfinite x then Nothing else Just x
    Nothing -> Nothing
evalExp' (EVal n) = pure n
evalExp :: Exp Double -> Maybe Int
evalExp e = do e' <- evalExp' e
               guard $ e' == fromInteger (round e') && e' > 0
               return $ floor e'


opsProd :: Int -> [[MatOp]]
opsProd k = forM [1..k] $ const [Add, Sub, Mul, Div]

exprs :: [Int] -> [Maybe Int]
exprs digits = do
    [d0, d1, d2, d3] <- map (map fromIntegral) $ permutations digits
    [op0, op1, op2] <- opsProd 3
    order <- permutations [0..2]
    return $ evalExp $ case order of
        [0, 1, 2] -> EOp op2 (EOp op1 (EOp op0 (EVal d0) (EVal d1)) (EVal d2)) (EVal d3)
        [0, 2, 1] -> EOp op1 (EOp op0 (EVal d0) (EVal d1)) (EOp op2 (EVal d2) (EVal d3))
        [1, 0, 2] -> EOp op2 (EOp op0 (EVal d0) (EOp op1 (EVal d1) (EVal d2))) (EVal d3)
        [1, 2, 0] -> EOp op0 (EVal d0) (EOp op2 (EOp op1 (EVal d1) (EVal d2)) (EVal d3))
        [2, 0, 1] -> EOp op1 (EOp op0 (EVal d0) (EVal d1)) (EOp op2 (EVal d2) (EVal d3))
        [2, 1, 0] -> EOp op0 (EVal d0) (EOp op1 (EVal d1) (EOp op2 (EVal d2) (EVal d3)))
    
allDigits :: [[Int]]
allDigits = [[a, b, c, d] |
    a <- [1..9],
    b <- [a + 1..9],
    c <- [b + 1..9],
    d <- [c + 1..9]]

checkScore = checkScore' 0 .  map head . group . sort . catMaybes where
    checkScore' score [] = score
    checkScore' score (h:t) = if score + 1 == h
        then checkScore' (score + 1) t
        else score

evalAll :: [(Int, String)]
evalAll = map (checkScore . exprs <:> map intToDigit) allDigits

max' (a, b) (c, d) = if a > c then (a, b) else (c, d)

euler093 :: IO String
euler093 = return $ snd $ foldr1 max' evalAll