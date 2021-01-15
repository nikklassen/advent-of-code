defmodule HashState do
    defstruct position: 0, skip_size: 0, nums: Enum.to_list(0..255)
end

defmodule Main do
    def compute_hash(length, state) do
        num_len = 256
        to_take = min(state.position + length, num_len) - state.position
        to_take_wrapped = length - to_take

        new_list =
            if to_take < length do
                {prefix, md_hd} = Enum.split(state.nums, state.position)
                {md_tl, rest} = Enum.split(prefix, to_take_wrapped)
                {rev_hd, rev_tl} = Enum.reverse(md_hd ++ md_tl) |> Enum.split(to_take)
                rev_tl ++ rest ++ rev_hd
            else
                {hd, rest} = Enum.split(state.nums, state.position)
                {middle, tl} = Enum.split(rest, length)
                hd ++ Enum.reverse(middle) ++ tl
            end

        %HashState{
            nums: new_list,
            position: rem(state.position + length + state.skip_size, num_len),
            skip_size: state.skip_size + 1
        }
    end

    def run() do
        File.read!("input")
        |> String.trim()
        |> String.split(",")
        |> Enum.map(&String.to_integer/1)
        |> List.foldl(%HashState{}, &Main.compute_hash/2)
        |> (fn (state) ->
                [a | [b | _]] = state.nums
                IO.puts(a * b)
            end).()
    end
end

Main.run()