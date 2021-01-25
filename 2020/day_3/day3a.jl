struct TreeField
    width::UInt32
    height::UInt32
    data::Array{Bool, 1}
end

function fromStdin()::TreeField
    height = 0
    width = 0
    data = Bool[]
    for line in eachline()
        height += 1
        width = length(line)
        for i in firstindex(line):lastindex(line)
            push!(data, line[i] != '.')
        end
    end
    TreeField(width, height, data)
end

function getIndex(field::TreeField, x, y)::Bool
    field.data[(y - 1) * field.width + mod1(x, field.width)]
end

function main()
    field = fromStdin()
    x = 1
    y = 1
    trees = 0
    while y <= field.height
        if getIndex(field, x, y)
            trees += 1
        end
        y += 1
        x += 3
    end

    println(trees)
end

main()