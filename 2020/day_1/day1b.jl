nums = Int32[]
for line in eachline()
    push!(nums, parse(Int32, line))
end

for num1 in nums
    for num2 in nums
        for num3 in nums
            if num1 + num2 + num3 == 2020
                println(num1 * num2 * num3)
                return
            end
        end
    end
end
