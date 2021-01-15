edges = Dict()
weights = Dict()
isabove = Set()
f = open("input")
for line in map(chomp, eachline(f))
    m = match(r"([a-z]+) \((\d+)\)(?: -> (.*))?", line)
    if m == nothing
        continue
    end
    bottom = m.captures[1]
    weight = parse(Int, m.captures[2])
    weights[bottom] = weight

    top = m.captures[3]
    if top == nothing
        edges[bottom] = []
    else
        above = split(top, ", ")
        map(p -> push!(isabove, p), above)
        edges[bottom] = above
    end
end
close(f)

cumulative_weights = Dict()
bottom = nothing
for vertex in keys(edges)
    if !in(vertex, isabove)
        bottom = vertex
        break
    end
end

function compute_weight(v)
    if haskey(cumulative_weights, v)
        return cumulative_weights[v]
    end
    weight = weights[v]

    other_weights = Dict()
    for above in edges[v]
        w = compute_weight(above)
        weight += w

        vertexes = get(other_weights, w, [])
        push!(vertexes, above)
        other_weights[w] = vertexes
    end
    if length(other_weights) > 1
        min_w = minimum(keys(other_weights))
        max_w = maximum(keys(other_weights))
        delta = max_w - min_w
        w = nothing
        if length(other_weights[min_w]) == 1
            w = weights[other_weights[min_w][1]] + delta
        else
            w = weights[other_weights[max_w][1]] - delta
        end
        println("$w")
    end

    return cumulative_weights[v] = weight
end
compute_weight(bottom)