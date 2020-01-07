module Solutions.Euler186 where
import Control.Monad
import Control.Monad.ST
import Data.Array
import Data.Array.ST

pmNodeIndex = 524287
bound = 10000000

plusmod :: Int -> Int -> Int
plusmod a b = (a + b) `mod` 1000000

laggedFib :: [Int]
laggedFib = runST $ do
    arr <- newArray_ (1, bound) :: ST s (STUArray s Int Int)
    forM_ [1..55] $ \n -> writeArray arr n $ plusmod (100003 - 200003 * n) (300007 * n * n * n)
    forM_ [56..bound] $ \n -> plusmod <$> readArray arr (n - 24) <*> readArray arr (n - 55) >>= writeArray arr n
    getElems arr

-- Roots have parent equal to their index in array and size is up to date only for them
data Node = Node { parent :: Int, size :: Int}
emptyNode n = Node n 1

nodeFind :: STArray s Int Node -> Int -> ST s Node
nodeFind arr x = readArray arr x >>= \node -> let p = parent node in if p == x
    then return node
    else nodeFind arr p >>= \node' -> writeArray arr p node' >> return node'

nodeSize :: STArray s Int Node -> Int -> ST s Int
nodeSize arr x = size <$> nodeFind arr x

nodeUnion :: STArray s Int Node -> Int -> Int -> ST s ()
nodeUnion arr x y = do
    Node xRoot xSize <- nodeFind arr x
    Node yRoot ySize <- nodeFind arr y
    if xRoot == yRoot
        then return ()
        else if xSize < ySize
                then writeArray arr xRoot (Node yRoot xSize) >> writeArray arr yRoot (Node yRoot $ xSize + ySize)
                else writeArray arr yRoot (Node xRoot ySize) >> writeArray arr xRoot (Node xRoot $ xSize + ySize)

pmConnected :: STArray s Int Node -> ST s Bool
pmConnected arr = (>= 990000) . size <$> nodeFind arr pmNodeIndex

runCalls :: [Int] -> Int -> STArray s Int Node -> ST s Int
runCalls (x:y:t) count arr = pmConnected arr >>= \done -> if done
    then return count
    else if x == y then runCalls t count arr
                   else nodeUnion arr x y >> runCalls t (count + 1) arr

calls :: Int
calls = runST $ newListArray (0, 999999) (map emptyNode [0..]) >>= runCalls laggedFib 0

euler186 :: IO String
euler186 = return $ show calls