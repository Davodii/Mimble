function recurse(a) does
    if a <= 0 do
        return 0
    end
    print(a)
    recurse(a-1)
end

recurse(10)