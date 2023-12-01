dick = {
        "one": 1,
        "two": 2,
        "three": 3,
        "four": 4,
        "five":5,
        "six":6,
        "seven":7,
        "eight":8,
        "nine":9,
        }

sum = 0
for x in open("./input.txt", "r"):
    a = 0
    b = 0
    if(a == 0):
        for i,y in enumerate(x):
            if(a != 0):
                break
            if(y.isdigit()):
                a = int(y)
                break
            for key in dick:
                if (x[i:i+len(key)] == key):
                    a = dick[key]
                    break
    if(b==0):
        for i,y in enumerate(x[::-1]):
            if(b != 0):
                break
            if(y.isdigit()):
                b = int(y)
                break
            for key in dick:
                if (x[len(x) - i -1 :len(x) - i+len(key)-1] == key):
                    b = dick[key]
                    break
        print(int(str(a) + str(b)), x)
        sum += int(str(a) + str(b))
print(sum)
