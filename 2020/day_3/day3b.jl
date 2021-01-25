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

function checkSlope(field::TreeField, xStride, yStride)
    x = 1
    y = 1
    trees = 0
    while y <= field.height
        if getIndex(field, x, y)
            trees += 1
        end
        y += yStride
        x += xStride
    end
    trees
end

function main()
    field = fromStdin()
    println(
        checkSlope(field, 1, 1) *
        checkSlope(field, 3, 1) *
        checkSlope(field, 5, 1) *
        checkSlope(field, 7, 1) *
        checkSlope(field, 1, 2))
end

main()