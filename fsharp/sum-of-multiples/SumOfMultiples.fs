module SumOfMultiples

let sum (numbers: int list) (upperBound: int): int =
        {0..upperBound + 1}
        |> Seq.filter (fun n -> numbers
                                |> List.exists (fun number -> n % number = 0))
        |> Seq.sum
        