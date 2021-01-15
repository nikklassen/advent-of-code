import qualified Data.Map as Map

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

data InfectionState = Clean | Weakened | Infected | Flagged deriving (Show, Eq)

data VirusState = VirusState {
    memory :: Map.Map (Int, Int) InfectionState
    , pos :: (Int, Int)
    , dir :: (Int, Int)
    , infected :: Int
    } deriving (Show)

addP (a, b) (x, y) = (a + x, b + y)

evolveInfection Clean = Weakened
evolveInfection Weakened = Infected
evolveInfection Infected = Flagged
evolveInfection Flagged = Clean

move Clean dir = turnLeft dir
move Weakened dir = dir
move Infected dir = turnRight dir
move Flagged (x, y) = (-x, -y)

proliferate :: VirusState -> VirusState
proliferate (state@VirusState { memory=m, pos=pos, dir=dir, infected=c }) =
    let infectionState = Map.findWithDefault Clean pos m
        newDir = move infectionState dir
        newState = evolveInfection infectionState
    in VirusState { memory = Map.insert pos newState m
    , pos = addP newDir pos
    , dir = newDir
    , infected = c + (if newState == Infected then 1 else 0)
    }

runVirus memory times =
    foldl (\s _ -> proliferate s) VirusState {
        memory = memory,
        pos = (0, 0),
        dir = (0, 1),
        infected = 0
    } [0..(times - 1)]

parseMemory :: [String] -> Map.Map (Int, Int) InfectionState
parseMemory rawMemory =
    let bx = (length rawMemory) `div` 2
        n = length $ rawMemory !! 0
        by = n `div` 2
    in Map.fromList [
        ((i, j), if ((rawMemory !! (n - j - by - 1)) !! (i + bx)) == '#' then Infected else Clean)
            | i <- [-bx..bx],
              j <- [-by..by]]

main = do
    content <- readFile "input"
    let rawMemory = lines content
    putStrLn $ show $ infected $ runVirus (parseMemory rawMemory) 10000000