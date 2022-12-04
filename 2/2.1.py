h = {"A":1,"B":2,"C":3,"X":1,"Y":2,"Z":3}

def out(a,b):
    a = h[a]
    b = h[b]

    if a == b: #draw
        return a + 3
    if (a == 1 and b == 2) or (a == 2 and b == 3) or (a == 3 and b == 1): #loss
        return a
    if (a == 1 and b == 3) or (a == 2 and b == 1) or (a == 3 and b == 2): #win
        return a + 6
    
print(sum([out(x[2],x[0]) for x in open("2/input.txt")]))