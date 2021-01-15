target = 277678

n = 3
a = 3
b = 2
A = [ 0 0 0
      0 1 0
      0 0 0 ]
while true
    while true
        next_cell = 0;
        for x = -1:1
            for y = -1:1
                if (a + x) < 1 || (a + x) > n || (b + y) < 1 || (b + y) > n
                    continue
                end
                next_cell += A(a + x, b + y);
            end
        end
        A(a, b) = next_cell;

        if A(a, b) > target
            fprintf('Value: %d\n', A(a, b))
            return
        end

        if a == n && b == 1
            break
        end

        if b == 1
            a += 1;
        elseif a == 1
            b -= 1;
        elseif b == n
            a -= 1;
        elseif a == n
            b += 1;
        end
    end

    col = zeros([n 1]);
    A = [col, A, col];
    row = zeros([1 n+2]);
    A = [row ; A ; row];
    n += 2;
    a = n;
    b = 2;
end
