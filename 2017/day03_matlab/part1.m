target = 277678
inner_square = floor(sqrt(target))
if (mod(inner_square, 2) == 0)
    inner_square -= 1
end
outer_square = ceil(sqrt(target))
if (mod(outer_square, 2) == 0)
    outer_square += 1
end
outer_bytes = target - inner_square**2
side_len = outer_square - 1
while outer_bytes > side_len
    outer_bytes -= side_len
end
fprintf('Moves: %d\n', outer_bytes - (side_len / 2) + ceil(inner_square / 2))
