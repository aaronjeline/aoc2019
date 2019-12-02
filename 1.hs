
main :: IO ()
main = do
  contents <- readFile "1.in"
  let l = lines contents
  print $ sum $ map (solve . read) l

solve :: Integer -> Integer
solve m =
  let base = fuel m in
    base + (fuel4fuel base)

fuel4fuel :: Integer -> Integer
fuel4fuel f =
  let new = fuel f in
    if new <= 0 then 0 else new + (fuel4fuel new)


fuel :: Integer -> Integer
fuel m = (div m 3) - 2
