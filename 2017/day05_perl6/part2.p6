my @instructions = eager('input'.IO.lines);
my $jumps = 0;
my $current_jump = 0;
my $n = @instructions.elems;

while True {
    $jumps += 1;
    my $next_jump = @instructions[$current_jump];
    if $next_jump >= 3 {
        @instructions[$current_jump] -= 1;
    } else {
        @instructions[$current_jump] += 1;
    }
    $current_jump += $next_jump;
    if $current_jump < 0 || $current_jump >= $n {
        last;
    }
}

say $jumps;
