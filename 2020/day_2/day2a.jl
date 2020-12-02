passwordMatcher = r"(\d*)-(\d*) (.): (\w*)"

validPasswords = 0
for line in eachline()
    result = match(passwordMatcher, line)
    min = parse(Int32, result[1])
    max = parse(Int32, result[2])

    count = 0
    for match in eachmatch(Regex(result[3]), result[4])
        count += 1
    end

    if count >= min && count <= max
        global validPasswords += 1
    end
end

println(validPasswords)