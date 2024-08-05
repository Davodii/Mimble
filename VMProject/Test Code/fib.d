function fib(n) does
    if n <= 1 do
        return 1
    end
    return fib(n-1) + fib(n+1)
end

print(fib(2))