function fib(n) does
    if n < 1 do
        return 1
    end
    return fib(n-1) + fib(n-2)
end

print("uh oh " + fib(6))