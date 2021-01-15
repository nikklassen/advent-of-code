{-# LANGUAGE DoAndIfThenElse #-}
import qualified Data.Set as Set
import Data.Set (Set, member, notMember)

turnLeft (1, 0) = (0, 1)
turnLeft (0, 1) = (-1, 0)
turnLeft (-1, 0) = (0, -1)
turnLeft (0, -1) = (1, 0)
turnLeft _ = undefined

turnRight (1, 0) = (0, -1)
turnRight (0, -1) = (-1, 0)
turnRight (-1, 0) = (0, 1)
turnRight (0, 1) = (1, 0)
turnRight _ = undefined

data VirusState = VirusState {
    memory :: Set (Int, Int)
    , pos :: (Int, Int)
    , dir :: (Int, Int)
    , infected :: Int
    } deriving (Show)

addP (a, b) (x, y) = (a + x, b + y)

proliferate :: VirusState -> VirusState
proliferate (state@VirusState { memory=m, pos=pos, dir=dir, infected=c }) =
    if member pos m then
        let newDir = turnRight dir in
        VirusState { memory = Set.delete pos m
        , pos = addP newDir pos
        , dir = newDir
        , infected=c
        }
    else
        let newDir = turnLeft dir in
        VirusState { memory = Set.insert pos m
        , pos = addP newDir pos
        , dir = newDir
        , infected = c + 1
        }

runVirus memory times =
    foldl (\s _ -> proliferate s) VirusState {
        memory = memory,
        pos = (0, 0),
        dir = (0, 1),
        infected = 0
    } [0..(times - 1)]

parseMemory :: [String] -> Set (Int, Int)
parseMemory rawMemory =
    let bx = (length rawMemory) `div` 2
        n = length $ rawMemory !! 0
        by = n `div` 2
    in Set.fromList [ (i, j) | i <- [-bx..bx],
                               j <- [-by..by],
                               ((rawMemory !! (n - j - by - 1)) !! (i + bx)) == '#']

main2 = do
    content <- readFile "input"
    let rawMemory = lines content
    putStrLn $ show $ infected $ runVirus (parseMemory rawMemory) 10000