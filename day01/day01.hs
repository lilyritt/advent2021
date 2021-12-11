module Main where

parseFile :: String -> [Int]
parseFile = map read . lines

sum3 :: Int -> Int -> Int -> Int
sum3 a b c = a + b + c

slidingSum :: [Int] -> [Int]
slidingSum (x:y:xs) = zipWith3 sum3 (x:y:xs) (x:xs) xs

larger :: [Int] -> Int
larger l = length . filter (uncurry (<)) . zip l $ tail l

main :: IO ()
main = do
  nums <- slidingSum . parseFile <$> readFile "input"
  print $ larger nums
