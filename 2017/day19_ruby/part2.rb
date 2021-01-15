def main()
    map = File.read('input').split("\n").map { |line| line.split('') }

    y = 0
    x = map[y].find_index '|'
    dir = :down
    steps = 0
    while true
        o = map[y][x]
        if o == '+' then
            if dir != :down && map[y-1][x] != ' ' then
                dir = :up
            elsif dir != :up && map[y+1][x] != ' ' then
                dir = :down
            elsif dir != :left && map[y][x+1] != ' ' then
                dir = :right
            elsif dir != :right && map[y][x-1] != ' ' then
                dir = :left
            else
                raise "Something went wrong"
            end
        elsif o == ' ' then
            # We're at the end
            break
        end
        steps += 1
        if dir == :down then
            y += 1
        elsif dir == :up then
            y -= 1
        elsif dir == :right then
            x += 1
        else
            x -= 1
        end
    end
    puts steps
end

main