module Main where

import Criterion.Main
import Data.List
import System.Random
import qualified Data.ByteString.Lazy.Char8 as B

readIntOnly = fmap fst . B.readInt

intPerLine = fmap readIntOnly . B.lines

-- compare the first and last element of a sliding window
frameCompare = (zipWith (<) <*>) . drop

-- AoC 2021 day 1
part n = sum . (fromEnum <$>) . frameCompare (n*2-1)

bart n = length . filter id . frameCompare (n*2-1)

main = do
    input <- B.readFile "../input.txt"
    let nums = sequence $ intPerLine input
    print $ part 1 <$> nums
    print $ bart 2 <$> nums

{-
randList n = sequence $ replicate n $ randomRIO (0,1::Int)

p n = part 2  n
b n = bart 2  n

main =
    let input = B.readFile "../input.txt"
    in let nums = sequence $ intPerLine <$> input
    in defaultMain [
    bgroup "day1" [ bench "part" $ nfIO p nums
                  , bench "bart" $ nfIO b nums
                  ]
    ]
-}

