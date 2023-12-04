killMe = []
stupid = [x for x in "*@#-/=%$+"]
for x in open('input.txt'):
    killMe += [[y for y in x[:-1]]]
y = 0
summm = 0
while y < len(killMe):
    x = 0
    try: print(killMe[y-1])
    except: pass
    print(killMe[y])
    try: print(killMe[y+1])
    except: pass
    while x < len(killMe[y]):
        if killMe[y][x].isdigit():
            meow = ""
            tmpx = x
            l = 0
            while x < len(killMe[y]) and killMe[y][x].isdigit(): 
                x += 1
                l += 1
            number = "".join(killMe[y][tmpx:x])
            for p in range(l+2):
                try:
                    meow += killMe[y-1][tmpx + p -1]
                except: pass
                try:
                    meow += killMe[y+1][tmpx + p -1]
                except: pass
            try:
                meow += killMe[y][tmpx - 1]
            except: pass
            try: 
                meow += killMe[y][x]
            except: pass
            if(any(e in meow for e in stupid)):
                summm += int(number) 
        else:
            x += 1
    y += 1
print(summm)
