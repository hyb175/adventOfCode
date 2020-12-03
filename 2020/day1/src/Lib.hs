module Lib
    ( day1
    ) where

import System.IO()
import qualified Data.HashSet as HashSet
import Data.Maybe

parseInput :: String -> [Int]
parseInput = map read . lines

findMatch :: [Int] -> HashSet.HashSet Int -> Int -> Maybe Int
findMatch (x:xs) counterPart output
    | HashSet.member (output - x) counterPart = Just x
    | null xs = Nothing
    | otherwise = findMatch xs (HashSet.insert x counterPart) output

part1 :: [Int] -> IO ()
part1 nums = do
    let output = findMatch nums HashSet.empty 2020

    let result = case output of Just x -> Just (x * (2020 - x))
                                Nothing -> Nothing

    putStrLn "Part1 result is -> "
    print result

findMatchForEach :: [Int] -> (Int, Int, Int)
findMatchForEach (num:rest) =
    case match of
        Just found -> (num, found, 2020 - num - found)
        Nothing -> findMatchForEach rest
    where match = findMatch rest HashSet.empty (2020 - num)


part2 :: [Int] -> IO ()
part2 nums = do
    let (numA, numB, numC) = findMatchForEach nums

    let result = numA * numB * numC

    putStrLn "Part2 result is -> "
    print result

day1 :: IO ()
day1 = do
    putStrLn "Read from input file..."
    contents <- readFile "input"

    let nums = parseInput contents
    part1 nums
    part2 nums
