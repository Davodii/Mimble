a = [1,2,3]
print(a)

pop(a,0)
# expect a : [2,3]
print(a)

pop(a,-1)
# expect a: [2]
print(a)

pop(a, 10)