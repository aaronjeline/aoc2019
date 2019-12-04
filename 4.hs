
main :: IO ()
main = do
  let r = [147981..691423]
  let passes = (filter (andmap [repeats_only2, increases]) . map split) r
  print $ length passes

split :: Int -> [Int]
split i = map readchar (show i)

readchar :: Char -> Int
readchar '1' = 1
readchar '2' = 2
readchar '3' = 3
readchar '4' = 4
readchar '5' = 5
readchar '6' = 6
readchar '7' = 7
readchar '8' = 8
readchar '9' = 9
readchar '0' = 0

andmap :: [a -> Bool] -> a -> Bool
andmap ps x = all (\p -> p x) ps

repeats :: [Int] -> Bool
repeats [] = False
repeats (_ : []) = False
repeats (x : y : xs) = if x == y then True else repeats (y : xs)

repeats_only2 :: [Int] -> Bool
repeats_only2 [] = False
repeats_only2 (x : xs) =
  if length group == 1 then True else repeats_only2 rest
  where (group,rest) = collect x xs

collect :: Eq a => a -> [a] -> ([a],[a])
collect _ [] = ([],[])
collect v (x : xs) =
  let (g,rst) = collect v xs in
    if v == x then
      (x:g, rst)
      else
      (g, x:rst)

increases :: [Int] -> Bool
increases [] = True
increases (_ : []) = True
increases (x : y : xs) = (x <= y) && increases (y : xs)
