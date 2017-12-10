use Bitwise, only_operators: true

defmodule HashState do
    defstruct position: 0, skip_size: 0, nums: Enum.to_list(0..255)
end

defmodule Main do
    def hash_step(length, state) do
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

    def compute_hash(lengths) do
        Enum.to_list(0..63)
        |> List.foldl(%HashState{}, fn(_, state) -> List.foldl(lengths, state, &Main.hash_step/2) end)
        |> (fn(state) -> Enum.chunk_every(state.nums, 16) end).()
        |> Enum.map(fn([hd | tl]) -> List.foldl(tl, hd, &(&1 ^^^ &2)) end)
    end

    def to_base_16(n) do
        Integer.to_string(n, 16)
        |> String.downcase()
        |> String.pad_leading(2, "0")
    end

    def run() do
        File.read!("input")
        |> String.trim()
        |> :binary.bin_to_list()
        |> Enum.concat([17, 31, 73, 47, 23])
        |> Main.compute_hash()
        |> Enum.map(&Main.to_base_16/1)
        |> Enum.join("")
        |> IO.puts()
    end
end

Main.run()