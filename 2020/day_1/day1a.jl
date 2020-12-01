nums = []
for line in eachline()
    num1 = parse(Int32, line)
    for num2 in nums
        if num2 + num1 == 2020
            println(num1 * num2)
            return
        end
    end
    push!(nums, num1)
end
