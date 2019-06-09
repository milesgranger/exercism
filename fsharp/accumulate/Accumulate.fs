module Accumulate

let accumulate (func: 'a -> 'b) (input: 'a list):
    input |> List.map func
