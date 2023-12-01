sum = 0
for x in open("./input.txt", "r"):
    a = 0
    b = 0
    for y in x:
        if(y.isdigit()):
            a = int(y)
    for y in x[::-1]:
        if(y.isdigit()):
            b = int(y)
    print(a,b, x)
    sum += b*10 + a
print(sum)
