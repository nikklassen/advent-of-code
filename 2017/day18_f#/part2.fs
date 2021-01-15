type MessageQueue(sent: int64, messages: int64 list) =
     member this.Sent = sent
     member this.isEmpty = Seq.isEmpty messages
     member this.Send(msg: int64) = MessageQueue(sent + 1L, messages @ (List.singleton msg))
     member this.Receive(): int64 * MessageQueue =
        match messages with
        | [] -> raise <| System.SystemException("Cannot receive from empty queue")
        | h::t -> (h, MessageQueue(sent, t))

type Program = {
    Instructions : string array
    Ip : int
    Registers : Map<string, int64>
    Messages : MessageQueue
}

let getValue (registers: Map<string, int64>) (v: string): int64 =
    let (success, intValue) = System.Int64.TryParse(v)
    if success then
        intValue
    else
        registers.TryFind(v) |> Option.defaultValue 0L

let eval (prog: Program): bool * int64 option * Program =
    assert (prog.Ip > 0 && prog.Ip < prog.Instructions.Length)
    let instruction = prog.Instructions.[prog.Ip].Split ' '
    let xValue = getValue prog.Registers instruction.[1]
    let yValue = if instruction.Length > 2 then getValue prog.Registers instruction.[2] else 0L
    let isWaiting = instruction.[0] = "rcv" && prog.Messages.isEmpty
    let toSend =
        if instruction.[0] = "snd" then
            Some <| getValue prog.Registers instruction.[1]
        else
            None
    let noopProg = { prog with Ip = prog.Ip + 1 }
    let program =
        match instruction.[0] with
        | "snd" -> noopProg
        | "set" ->
            { prog with Ip = prog.Ip + 1; Registers = prog.Registers.Add(instruction.[1], yValue) }
        | "add" ->
            let newValue = xValue + yValue
            { prog with Ip = prog.Ip + 1; Registers = prog.Registers.Add(instruction.[1], newValue) }
        | "mul" ->
            let newValue = xValue * yValue
            { prog with Ip = prog.Ip + 1; Registers = prog.Registers.Add(instruction.[1], newValue) }
        | "mod" ->
            let newValue = xValue % yValue
            { prog with Ip = prog.Ip + 1; Registers = prog.Registers.Add(instruction.[1], newValue) }
        | "rcv" ->
            if prog.Messages.isEmpty then
                prog
            else
                let (msg, queue) = prog.Messages.Receive()
                { prog with Ip = prog.Ip + 1; Registers = prog.Registers.Add(instruction.[1], msg); Messages = queue }
        | "jgz" ->
            if xValue > 0L then
                { prog with Ip = prog.Ip + (int yValue) }
            else
                noopProg
        | i -> raise <| System.InvalidOperationException("Error, unknown instruction " + i)
    (isWaiting, toSend, program)

let rec run (prog0: Program) (prog1: Program) =
    let (waiting0, out0, newProg0) = eval prog0
    let (waiting1, out1, newProg1) = eval prog1
    let sendMessage (ms: MessageQueue) v = ms.Send(v)
    if waiting0 && waiting1 then
        // The this actually keeps track of how many messages were sent TO prog 0, i.e. by prog 1
        prog0.Messages.Sent
    else
        run { newProg0 with Messages = Option.fold sendMessage newProg0.Messages out1 }
            { newProg1 with Messages = Option.fold sendMessage newProg1.Messages out0 }

let main =
    let instructions = System.IO.File.ReadAllLines("input") |> Seq.toArray
    let prog0 = { Instructions = instructions; Ip = 0; Registers = Map.ofList [ ("p", 0L) ];  Messages = MessageQueue(0L, List.empty) }
    let prog1 = { Instructions = instructions; Ip = 0; Registers = Map.ofList [ ("p", 1L) ];  Messages = MessageQueue(0L, List.empty) }
    run prog0 prog1
    |> printfn "Result: %d"