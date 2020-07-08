module Utils.Input
    ( getInput
    , getSolutionFromTxt
    , getSolutionFromTex
    ) where
import Text.Printf
import System.Directory
import System.Environment
import System.FilePath(joinPath)

getFile :: FilePath -> Int -> IO String
getFile ext problemNo = do
    execPath <- getExecutablePath
    readFile (joinPath [execPath, "..", "..", "..", "..", "..", "..", ext, printf "Euler%03d.%s" problemNo ext])

getInput :: Int -> IO String
getInput = getFile "in"

-- Convention for txt files: last line of text file contains solution to the problem.
getSolutionFromTxt :: Int -> IO String
getSolutionFromTxt problemNo =
    putStrLn "Reading solution from text file" >>
    last . filter (not . null) . lines <$> getFile "txt" problemNo

-- Convention for tex files: document ends like this:
-- \vspace{5ex}
-- \textbf{Answer:}
-- XYZ
-- \end{document}
-- Unless tex file is only explaining computations done.
getSolutionFromTex :: Int -> IO String
getSolutionFromTex problemNo =
    putStrLn "Reading solution from tex file" >>
    last . init . filter (not . null) . lines <$> getFile "tex" problemNo