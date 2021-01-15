let getValue (registers: Map<string, int64>) (v: string): int64 =
    let (success, intValue) = System.Int64.TryParse(v)
    if success then
        intValue
    else
        registers.TryFind(v) |> Option.defaultValue 0L

let rec eval (instructions: string array) (ip: int) (registers: Map<string, int64>) (freq: int64): int64 =
    let instruction = instructions.[ip].Split ' '
    let xValue = registers.TryFind(instruction.[1]) |> Option.defaultValue 0L
    let yValue = if instruction.Length > 2 then getValue registers instruction.[2] else 0L
    match instruction.[0] with
        | "snd" ->
            eval instructions (ip + 1) registers (getValue registers instruction.[1])
        | "set" ->
            eval instructions (ip + 1) (registers.Add(instruction.[1], yValue)) freq
        | "add" ->
            let newValue = xValue + yValue
            eval instructions (ip + 1) (registers.Add(instruction.[1], newValue)) freq
        | "mul" ->
            let newValue = xValue * yValue
            eval instructions (ip + 1) (registers.Add(instruction.[1], newValue)) freq
        | "mod" ->
            let newValue = xValue % yValue
            eval instructions (ip + 1) (registers.Add(instruction.[1], newValue)) freq
        | "rcv" ->
            if xValue <> 0L then
                freq
            else
                eval instructions (ip + 1) registers freq
        | "jgz" ->
            if xValue > 0L then
                eval instructions (ip + (int yValue)) registers freq
            else
                eval instructions (ip + 1) registers freq
        | i -> printfn "Error, unknown instruction %s" i; 0L

let main =
    eval (System.IO.File.ReadAllLines("input") |> Seq.toArray) 0 Map.empty 0L
    |> printfn "Result: %d"