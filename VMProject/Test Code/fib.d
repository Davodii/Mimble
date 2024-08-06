function recursiveFib(n) does
    if n <= 1 do
        return 1
    end
    return recursiveFib(n-1) + recursiveFib(n-2)
end

function linearFib(n) does
    count = 0
    f1 = 1
    f2 = 1
    fn = 1
    while count < n do
        fn = f1 + f2
        f2 = f1
        f1 = fn
        count = count + 1
    end
    return fn
end

# Change this to get a higher fib number
n = 10

# Recursively
print("The " + n + "th fib number is: " + recursiveFib(n))

# Linearly
print("The " + n + "th fib number is: " + linearFib(n))