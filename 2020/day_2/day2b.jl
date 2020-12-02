passwordMatcher = r"(\d*)-(\d*) (.): (\w*)"

validPasswords = 0
for line in eachline()
    result = match(passwordMatcher, line)
    firstIndex = parse(Int32, result[1])
    secondIndex = parse(Int32, result[2])
    char = result[3]
    password = result[4]
    
    if xor(password[firstIndex] == char[1], password[secondIndex] == char[1])
        global validPasswords += 1
    end
end

println(validPasswords)