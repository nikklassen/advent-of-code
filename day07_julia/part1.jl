edges = Dict()
isabove = Set()
f = open("input")
for line in map(chomp, eachline(f))
    m = match(r"([a-z]+) \(\d+\)(?: -> (.*))?", line)
    if m == nothing
        continue
    end
    bottom = m.captures[1]
    top = m.captures[2]
    if top == nothing
        edges[bottom] = []
    else
        above = split(top, ", ")
        map(p -> push!(isabove, p), above)
        edges[bottom] = above
    end
end
close(f)

for vertex in keys(edges)
    if !in(vertex, isabove)
        println("$vertex")
    end
end