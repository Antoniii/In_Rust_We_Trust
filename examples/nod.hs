import System.Environment (getArgs)

nod :: Int -> Int -> Int
nod x y = if y /= 0 then nod y (x `mod` y) else x

nok :: Int -> Int -> Int
nok x y = (x*y) `div` (nod x y)


main :: IO ()
main = do args <- getArgs
          case args of [a, b] -> do putStr "НОД: "
                                    print $ nod (read a) (read b)
                                    putStr "НОК: "
                                    print $ nok (read a) (read b)
                       _      -> putStrLn "Некорректные данные, \
                                          \программа ожидает два натуральных числа a и b"

-- запуск: ./nod 567 456